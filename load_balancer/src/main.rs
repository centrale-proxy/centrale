use async_trait::async_trait;
use bytes::Bytes;
use common::payload::{CheckIn2, CheckOut2, WriterPayload};
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
    writer_socket: Arc<UdpSocket>,
    writer_addr: SocketAddr,
}

const MAX_LOGGED_RESPONSE_BODY_BYTES: usize = 8 * 1024;

pub struct RequestCtx {
    pub x_id: String,
    pub response_body: Vec<u8>,
    pub response_body_truncated: bool,
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
        // send_request_bytes_to_writer(&self.writer_socket, self.writer_addr, request_bytes);
        let request_bytes = request_head_to_bytes(session);
        let ip = client_for_logging(session);
        // pass the ctx's x_id into the checkin instead of generating it inside
        let checkin = CheckIn2::new(Some(ip), request_bytes, ctx.x_id.clone());
        send_request_bytes_to_writer_2(&self.writer_socket, self.writer_addr, checkin);
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

        let checkout = match e {
            Some(err) => CheckOut2::new(Some(status), Some(err.to_string()), ctx.x_id.clone()),
            None if status != 200 => CheckOut2::new(
                Some(status),
                response_body_for_logging(ctx).or_else(|| Some("err".to_string())),
                ctx.x_id.clone(),
            ),
            None => CheckOut2::new(Some(status), None, ctx.x_id.clone()),
        };
        send_request_bytes_to_writer_checkout(&self.writer_socket, self.writer_addr, checkout);
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

fn request_head_to_bytes(session: &Session) -> Vec<u8> {
    session.downstream_session.to_h1_raw().to_vec()
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

fn send_request_bytes_to_writer(socket: &UdpSocket, addr: SocketAddr, request_bytes: Vec<u8>) {
    match socket.send_to(&request_bytes, addr) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::WouldBlock => {}
        Err(err) => error!("Unable to send load balancer bytes to writer: {}", err),
    }
}

fn send_request_bytes_to_writer_2(socket: &UdpSocket, addr: SocketAddr, checkin: CheckIn2) {
    // Wrap in the enum variant, then serialize as JSON
    let payload = WriterPayload::CheckIn2(checkin);

    let request_bytes = match serde_json::to_vec(&payload) {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Unable to serialize payload: {}", err);
            return;
        }
    };

    match socket.send_to(&request_bytes, addr) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::WouldBlock => {}
        Err(err) => error!("Unable to send load balancer bytes to writer: {}", err),
    }
}

fn send_request_bytes_to_writer_checkout(
    socket: &UdpSocket,
    addr: SocketAddr,
    checkout: CheckOut2,
) {
    // Wrap in the enum variant, then serialize as JSON
    let payload = WriterPayload::CheckOut2(checkout);

    let request_bytes = match serde_json::to_vec(&payload) {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Unable to serialize payload: {}", err);
            return;
        }
    };

    match socket.send_to(&request_bytes, addr) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::WouldBlock => {}
        Err(err) => error!("Unable to send load balancer bytes to writer: {}", err),
    }
}

fn main() {
    let mut logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    logger.filter_module("load_balancer", LevelFilter::Info);
    logger.init();
    dotenv().ok();

    let centrale_upstream_address = get_centrale_upstream_address();
    let writer_addr: SocketAddr = CentraleConfig::WRITER_SERVER_ADDRESS.parse().unwrap();
    let writer_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    writer_socket.set_nonblocking(true).unwrap();
    let writer_socket = Arc::new(writer_socket);

    info!(
        "Starting Pingora load balancer on 0.0.0.0:443 -> {}",
        centrale_upstream_address
    );
    info!("Writer UDP logging enabled: {}", writer_addr);

    let cert_chain_path = CentraleConfig::cert_pub_key();
    let cert_private_key_path = CentraleConfig::cert_private_key();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut proxy_service = http_proxy_service(
        &server.configuration,
        LoadBalancer {
            centrale_upstream_address,
            writer_socket,
            writer_addr,
        },
    );

    let mut tls_settings =
        TlsSettings::intermediate(&cert_chain_path, &cert_private_key_path).unwrap();
    tls_settings.enable_h2();

    proxy_service.add_tls_with_settings("0.0.0.0:443", None, tls_settings);

    server.add_service(proxy_service);
    server.run_forever();
}

fn get_centrale_upstream_address() -> String {
    match std::env::var("CENTRALE_UPSTREAM_ADDRESS") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            let bind_address = CentraleConfig::get("SERVER_ADDRESS");

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
