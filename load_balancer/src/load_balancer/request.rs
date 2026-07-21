mod helpers;

use self::helpers::{
    client_ip, redirect_http_to_https, reject_ip_literal_host, request_host, request_host_and_path,
    should_route_to_www,
};
use crate::{
    load_balancer::{LoadBalancer, RequestCtx},
    read_full_body::read_full_body,
};
use common::payload::{CentralePing, CentralePingInput, CheckIn};
use pingora::{
    http::ResponseHeader,
    prelude::{HttpPeer, Result, Session},
};

pub async fn request_filter(
    load_balancer: &LoadBalancer,
    session: &mut Session,
    ctx: &mut RequestCtx,
) -> Result<bool> {
    // GET BASIC DATA
    let (host, path_and_query) = request_host_and_path(session);
    // NO DIRECT IP ACCESS
    if reject_ip_literal_host(session, host.as_deref()).await? {
        return Ok(true);
    }
    // REDIRECT HTTP TO HTTPS
    if redirect_http_to_https(
        session,
        load_balancer.force_https_redirect,
        host.as_deref(),
        &path_and_query,
    )
    .await?
    {
        return Ok(true);
    }

    // GET IP
    let ip = client_ip(session);

    // SEND PING OR CHECKIN
    if path_and_query == "/api/ping" {
        // IT'S PING
        // tbd extract url and counter
        ctx.is_ping = true;

        let body = read_full_body(session).await?;
        match serde_json::from_slice::<CentralePingInput>(&body) {
            Ok(ping) => {
                let ip_and_port = ip.for_logging().to_string();
                let ip_only = ip_and_port
                    .split(':')
                    .next()
                    .unwrap_or(&ip_and_port)
                    .to_string();

                let ping2 = CentralePing::new(ping.counter, &ping.url, ip_only, host);

                load_balancer.writer.send_ping(ping2);
            }
            Err(e) => {
                eprintln!("bad JSON for ping: {e}");
            }
        }

        // RETURN PING IMMEDIATELY
        let mut response = ResponseHeader::build(200, Some(2))?;
        //  response.insert_header("Location", location)?;
        response.insert_header("Content-Length", "0")?;
        response.insert_header("Cache-Control", "no-store")?;

        session
            .write_response_header(Box::new(response), true)
            .await?;
        return Ok(true);
    } else {
        // SEND CheckIn
        let request_bytes = session.downstream_session.to_h1_raw().to_vec();
        let checkin = CheckIn::new(ip, request_bytes, ctx.x_id.clone(), host.clone());
        load_balancer.writer.send_checkin(checkin);
    }

    Ok(false)
}

pub async fn upstream_peer(
    load_balancer: &LoadBalancer,
    session: &mut Session,
) -> Result<Box<HttpPeer>> {
    let host = request_host(session);

    let route_to_www = load_balancer.www_upstream_address.is_some()
        && should_route_to_www(host.as_deref(), load_balancer.www_host.as_deref());

    let upstream_address = if route_to_www {
        load_balancer.www_upstream_address.as_deref().unwrap()
    } else {
        load_balancer.centrale_upstream_address.as_str()
    };

    let peer = HttpPeer::new(upstream_address, false, "localhost".to_string());
    Ok(Box::new(peer))
}
