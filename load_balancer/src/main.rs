use async_trait::async_trait;
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

struct LoadBalancer {
    centrale_upstream_address: String,
    writer_socket: Arc<UdpSocket>,
    writer_addr: SocketAddr,
}

#[async_trait]
impl ProxyHttp for LoadBalancer {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let request_bytes = request_head_to_bytes(session);
        send_request_bytes_to_writer(&self.writer_socket, self.writer_addr, request_bytes);
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
}

fn request_head_to_bytes(session: &Session) -> Vec<u8> {
    session.downstream_session.to_h1_raw().to_vec()
}

fn send_request_bytes_to_writer(socket: &UdpSocket, addr: SocketAddr, request_bytes: Vec<u8>) {
    match socket.send_to(&request_bytes, addr) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::WouldBlock => {}
        Err(err) => error!("Unable to send load balancer bytes to writer: {}", err),
    }
}

fn main() {
    env_logger::init();
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
