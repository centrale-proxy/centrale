use async_trait::async_trait;
use bytes::Bytes;
use common::payload::{CheckIn, CheckOut, WriterPayload};
use config::CentraleConfig;
use dotenvy::dotenv;
use log::{LevelFilter, error, info};
use pingora::{
    listeners::tls::TlsSettings,
    prelude::{HttpPeer, ProxyHttp, Result, Server, Session, http_proxy_service},
};
use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};
use uuid::Uuid;

struct LoadBalancer {
    centrale_upstream_address: String,
    writer: WriterClient,
}

const MAX_LOGGED_RESPONSE_BODY_BYTES: usize = 8 * 1024;

pub struct RequestCtx {
    pub x_id: String,
    pub response_body: Vec<u8>,
    pub response_body_truncated: bool,
}

struct WriterClient {
    socket: Arc<UdpSocket>,
    addr: SocketAddr,
}

impl WriterClient {
    fn new(socket: Arc<UdpSocket>, addr: SocketAddr) -> Self {
        Self { socket, addr }
    }

    fn send_checkin(&self, checkin: CheckIn) {
        self.send(WriterPayload::CheckIn(checkin));
    }

    fn send_checkout(&self, checkout: CheckOut) {
        self.send(WriterPayload::CheckOut(checkout));
    }

    fn send(&self, payload: WriterPayload) {
        let request_bytes = match serde_json::to_vec(&payload) {
            Ok(bytes) => bytes,
            Err(err) => {
                error!("Unable to serialize payload: {}", err);
                return;
            }
        };

        match self.socket.send_to(&request_bytes, self.addr) {
            Ok(_) => {}
            Err(err) if err.kind() == ErrorKind::WouldBlock => {}
            Err(err) => error!("Unable to send load balancer bytes to writer: {}", err),
        }
    }
}

#[async_trait]
impl ProxyHttp for LoadBalancer {
    type CTX = RequestCtx;

    fn new_ctx(&self) -> Self::CTX {
        RequestCtx {
            x_id: Uuid::new_v4().to_string(),
            response_body: Vec::new(),
            response_body_truncated: false,
        }
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let request_bytes = session.downstream_session.to_h1_raw().to_vec();
        let ip = client_for_logging(session);
        let checkin = CheckIn::new(Some(ip), request_bytes, ctx.x_id.clone());
        self.writer.send_checkin(checkin);
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = HttpPeer::new(
            self.centrale_upstream_address.as_str(),
            false,
            "localhost".to_string(),
        );
        Ok(Box::new(peer))
    }

    fn response_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<Bytes>,
        _end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> Result<Option<std::time::Duration>>
    where
        Self::CTX: Send + Sync,
    {
        if let Some(chunk) = body.as_ref() {
            let remaining = MAX_LOGGED_RESPONSE_BODY_BYTES.saturating_sub(ctx.response_body.len());

            if remaining > 0 {
                let bytes_to_copy = remaining.min(chunk.len());
                ctx.response_body.extend_from_slice(&chunk[..bytes_to_copy]);
            }

            if chunk.len() > remaining {
                ctx.response_body_truncated = true;
            }
        }

        Ok(None)
    }

    async fn logging(
        &self,
        session: &mut Session,
        e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        let status = session.response_written().map_or(0, |r| r.status.as_u16());

        let checkout = build_checkout(status, e, ctx);
        self.writer.send_checkout(checkout);
    }
}

fn client_for_logging(session: &Session) -> String {
    if let Some(forwarded_for) = session
        .req_header()
        .headers
        .get("forwarded")
        .and_then(|value| value.to_str().ok())
        .and_then(extract_forwarded_for)
    {
        return forwarded_for;
    }

    session
        .client_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn extract_forwarded_for(forwarded_header: &str) -> Option<String> {
    for element in forwarded_header.split(',') {
        for pair in element.split(';') {
            let Some((key, value)) = pair.split_once('=') else {
                continue;
            };

            if !key.trim().eq_ignore_ascii_case("for") {
                continue;
            }

            let value = value.trim();
            if value.is_empty() {
                continue;
            }

            let value = value
                .strip_prefix('"')
                .and_then(|inner| inner.strip_suffix('"'))
                .unwrap_or(value);

            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

fn build_checkout(status: u16, e: Option<&pingora::Error>, ctx: &RequestCtx) -> CheckOut {
    match e {
        Some(err) => CheckOut::new(Some(status), Some(err.to_string()), ctx.x_id.clone()),
        None if status != 200 => CheckOut::new(
            Some(status),
            response_body_for_logging(ctx).or_else(|| Some("err".to_string())),
            ctx.x_id.clone(),
        ),
        None => CheckOut::new(Some(status), None, ctx.x_id.clone()),
    }
}

fn response_body_for_logging(ctx: &RequestCtx) -> Option<String> {
    if ctx.response_body.is_empty() {
        return None;
    }

    let mut body = String::from_utf8_lossy(&ctx.response_body)
        .trim()
        .to_string();
    if body.is_empty() {
        return None;
    }

    if ctx.response_body_truncated {
        body.push_str(" …[truncated]");
    }

    Some(body)
}

fn main() {
    let mut logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    logger.filter_module("load_balancer", LevelFilter::Info);
    logger.init();
    dotenv().ok();

    // let centrale_upstream_address = get_centrale_upstream_address();
    let centrale_upstream_address = CentraleConfig::get("CENTRALE_ADDRESS");
    let writer_addr: SocketAddr = CentraleConfig::WRITER_SERVER_ADDRESS.parse().unwrap();
    let writer_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    writer_socket.set_nonblocking(true).unwrap();
    let writer_socket = Arc::new(writer_socket);
    let writer = WriterClient::new(writer_socket, writer_addr);

    info!(
        "Starting Pingora load balancer on 0.0.0.0:443 -> {}",
        centrale_upstream_address
    );
    info!("Writer UDP logging enabled: {}", writer_addr);
    // ADD SSL
    let cert_chain_path = CentraleConfig::cert_pub_key();
    let cert_private_key_path = CentraleConfig::cert_private_key();
    // CREATE SERVER
    let mut server = Server::new(None).unwrap();
    server.bootstrap();
    // ADD SERVICE
    let mut proxy_service = http_proxy_service(
        &server.configuration,
        LoadBalancer {
            centrale_upstream_address,
            writer,
        },
    );
    // ADD SETTINGS
    let mut tls_settings =
        TlsSettings::intermediate(&cert_chain_path, &cert_private_key_path).unwrap();
    tls_settings.enable_h2();
    proxy_service.add_tls_with_settings("0.0.0.0:443", None, tls_settings);
    // ADD SERVICE
    server.add_service(proxy_service);
    server.run_forever();
}
/*
fn get_centrale_upstream_address() -> String {
    match std::env::var("CENTRALE_UPSTREAM_ADDRESS") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            let bind_address = CentraleConfig::get("CENTRALE_ADDRESS");

            if let Some(port) = bind_address.strip_prefix("0.0.0.0:") {
                return format!("127.0.0.1:{port}");
            }

            if let Some(port) = bind_address.strip_prefix("[::]:") {
                return format!("127.0.0.1:{port}");
            }

            bind_address
        }
    }
}
 */
