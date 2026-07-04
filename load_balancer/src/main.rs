mod request;
mod response;

use async_trait::async_trait;
use bytes::Bytes;
use common::payload::{CheckIn, CheckOut, WriterPayload};
use config::CentraleConfig;
use dotenvy::dotenv;
use log::{error, info};
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

use crate::{request::client_for_logging, response::build_checkout};

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

fn main() {
    env_logger::init();
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
