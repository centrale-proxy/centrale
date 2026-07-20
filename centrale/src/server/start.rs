use crate::proxy::wildcard::create_client::create_client_with_cert;
use crate::server::rate_limiter::get_rate_limiter_config;
use crate::server::routes::routes;
// server::log::log_middleware
use actix_cors::Cors;
use actix_governor::Governor;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbPool;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

fn origin_is_allowed(origin: &str, base_domain: &str) -> bool {
    let without_scheme = origin
        .strip_prefix("https://")
        .or_else(|| origin.strip_prefix("http://"));

    let Some(without_scheme) = without_scheme else {
        return false;
    };

    let host_and_port = without_scheme.split('/').next().unwrap_or_default();
    let host = host_and_port.split(':').next().unwrap_or_default();

    host == base_domain || host.ends_with(&format!(".{}", base_domain))
}

fn cors_for_domain(base_domain: &str) -> Cors {
    let base_domain = base_domain.to_string();

    Cors::default()
        .allowed_origin_fn(move |origin, _req_head| {
            origin
                .to_str()
                .map(|origin| origin_is_allowed(origin, &base_domain))
                .unwrap_or(false)
        })
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600)
}

#[actix_web::main]
pub async fn start_server(db: DbPool) -> std::io::Result<()> {
    // RATE LIMITING SETTINGS
    let governor_conf = get_rate_limiter_config();
    // SET UP CONNECTION TO WRITER
    //let addr_0: SocketAddr = "0.0.0.0:0".parse().unwrap();
    // let socket = UdpSocket::bind(addr_0)?; // OS picks a port
    // let socket_arc = Arc::new(Mutex::new(socket));
    // let addr: SocketAddr = CentraleConfig::WRITER_SERVER_ADDRESS.parse().unwrap();
    // CREATE CLIENT WITH CERT
    let client = create_client_with_cert().unwrap();

    let proxy_is_443 = CentraleConfig::get("CENTRALE_IS_443");
    let cors_domain = CentraleConfig::get("DOMAIN");
    //
    if proxy_is_443 == "true".to_string() {
        // SSL //
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file(CentraleConfig::cert_private_key(), SslFiletype::PEM)?;
        builder.set_certificate_chain_file(CentraleConfig::cert_pub_key())?;
        // SERVER ITSELF
        HttpServer::new(move || {
            let cors = cors_for_domain(&cors_domain);

            App::new()
                .configure(routes)
                .app_data(web::Data::new(db.clone()))
                /*
                .wrap_fn({
                    let socket_2 = socket_arc.clone();
                    move |req, srv| log_middleware(req, srv, socket_2.clone(), addr)
                })
                 */
                .wrap(Governor::new(&governor_conf))
                .wrap(cors)
                .app_data(web::Data::new(client.clone()))
        })
        .workers(CentraleConfig::PROXY_SERVER_WORKERS)
        .bind_openssl(CentraleConfig::get("CENTRALE_ADDRESS"), builder)? //
        .run()
        .await
    } else {
        // NOT PORT 443
        // SERVER ITSELF
        HttpServer::new(move || {
            let cors = cors_for_domain(&cors_domain);

            App::new()
                .configure(routes)
                .app_data(web::Data::new(db.clone()))
                /*
                .wrap_fn({
                    let socket_2 = socket_arc.clone();
                    move |req, srv| log_middleware(req, srv, socket_2.clone(), addr)
                })
                */
                .wrap(Governor::new(&governor_conf))
                .wrap(cors)
                .app_data(web::Data::new(client.clone()))
        })
        .workers(CentraleConfig::PROXY_SERVER_WORKERS)
        .bind(CentraleConfig::get("CENTRALE_ADDRESS"))? //
        .run()
        .await
    }
}
