use crate::{
    connect_to_writer::WriterClient, error::LoadBalancerError, load_balancer::LoadBalancer,
};
use config::CentraleConfig;
use log::info;
use pingora::{
    listeners::tls::TlsSettings,
    prelude::{Server, http_proxy_service},
};
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

pub fn start() -> Result<(), LoadBalancerError> {
    // CENTRALE ADDRESS
    let centrale_upstream_address = CentraleConfig::get("CENTRALE_ADDRESS");

    // OPTIONAL WWW ROUTING
    let www_upstream_address = Some(CentraleConfig::get("WWW_UPSTREAM_ADDRESS"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let www_host = std::env::var("DOMAIN").ok().map(|domain| {
        let normalized_domain = domain.trim().trim_end_matches('.').to_ascii_lowercase();
        format!("www.{}", normalized_domain)
    });

    // WRITER ADDRESS
    let writer_addr: SocketAddr = CentraleConfig::get("WRITER_SERVER_ADDRESS")
        .parse()
        .unwrap();

    // START SOCKET
    let writer_socket = UdpSocket::bind("0.0.0.0:0")?;

    // CONNECT TO WRITER
    writer_socket.set_nonblocking(true)?;
    let writer_socket = Arc::new(writer_socket);
    let writer = WriterClient::new(writer_socket, writer_addr);

    info!(
        "Starting Pingora load balancer on 0.0.0.0:443 -> {}",
        centrale_upstream_address
    );
    if let Some(www_upstream) = www_upstream_address.as_deref() {
        let www_host_for_log = www_host.as_deref().unwrap_or("www.*");
        info!(
            "WWW routing enabled: {} -> {}",
            www_host_for_log, www_upstream
        );
    } else {
        info!("WWW routing disabled (set WWW_UPSTREAM_ADDRESS to enable)");
    }
    info!("Writer UDP logging enabled: {}", writer_addr);

    // ADD SSL
    let cert_chain_path = CentraleConfig::cert_pub_key();
    let cert_private_key_path = CentraleConfig::cert_private_key();

    // CREATE SERVER
    let mut server = Server::new(None)?;
    server.bootstrap();

    // ADD SERVICE
    let mut proxy_service = http_proxy_service(
        &server.configuration,
        LoadBalancer {
            centrale_upstream_address,
            www_upstream_address,
            www_host,
            writer,
        },
    );

    // ADD SETTINGS
    let mut tls_settings = TlsSettings::intermediate(&cert_chain_path, &cert_private_key_path)?;
    tls_settings.enable_h2();
    proxy_service.add_tls_with_settings("0.0.0.0:443", None, tls_settings);

    // ADD SERVICE
    server.add_service(proxy_service);

    // START
    server.run_forever();
}
