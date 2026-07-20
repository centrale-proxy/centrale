use crate::{
    connect_to_writer::WriterClient, read_full_body::read_full_body, request::client_ip,
    response::build_checkout,
};
use async_trait::async_trait;
use bytes::Bytes;
use common::payload::{CentralePing, CentralePingInput, CheckIn};
use pingora::{
    http::ResponseHeader,
    prelude::{HttpPeer, ProxyHttp, Result, Session},
};
use std::net::IpAddr;
use uuid::Uuid;

pub struct LoadBalancer {
    pub centrale_upstream_address: String,
    pub www_upstream_address: Option<String>,
    pub www_host: Option<String>,
    pub force_https_redirect: bool,
    pub writer: WriterClient,
}

const MAX_LOGGED_RESPONSE_BODY_BYTES: usize = 8 * 1024;

pub struct RequestCtx {
    pub x_id: String,
    pub response_body: Vec<u8>,
    pub response_body_truncated: bool,
    pub is_ping: bool,
}

#[async_trait]
impl ProxyHttp for LoadBalancer {
    type CTX = RequestCtx;

    fn new_ctx(&self) -> Self::CTX {
        RequestCtx {
            x_id: Uuid::new_v4().to_string(),
            response_body: Vec::new(),
            response_body_truncated: false,
            is_ping: false,
        }
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let request_bytes = session.downstream_session.to_h1_raw().to_vec();

        let (host, path_and_query) = {
            let req = session.req_header();

            let host = req
                .uri
                .authority()
                .map(|a| a.as_str().to_string())
                .or_else(|| {
                    req.headers
                        .get("host")
                        .and_then(|v| v.to_str().ok())
                        .map(str::to_string)
                });

            let path_and_query = request_path_and_query(req.uri.path(), req.uri.query());
            (host, path_and_query)
        };

        let ip = client_ip(session);

        // SEND PING OR CHECKIN
        if path_and_query == "/api/ping" {
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

                    self.writer.send_ping(ping2);
                }
                Err(e) => {
                    // respond 400 and short-circuit the proxy
                    //session.respond_error(400).await?;
                    eprintln!("bad JSON: {e}");
                    //return Ok(true); // true = response already sent, stop processing
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
            let checkin = CheckIn::new(ip, request_bytes, ctx.x_id.clone(), host.clone());
            self.writer.send_checkin(checkin);
        }

        // Do not proxy requests addressed to an IP literal.
        if host.as_deref().is_some_and(is_ip_literal_host) {
            let mut response = ResponseHeader::build(421, Some(3))?;
            response.insert_header("Content-Length", "0")?;
            response.insert_header("Cache-Control", "no-store")?;

            session
                .write_response_header(Box::new(response), true)
                .await?;
            return Ok(true);
        }

        if self.force_https_redirect && is_plain_http_request(session) {
            if let Some(location) = build_https_redirect_location(host.as_deref(), &path_and_query)
            {
                let mut response = ResponseHeader::build(308, Some(3))?;
                response.insert_header("Location", location)?;
                response.insert_header("Content-Length", "0")?;
                response.insert_header("Cache-Control", "no-store")?;

                session
                    .write_response_header(Box::new(response), true)
                    .await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let host = request_host(session);

        let route_to_www = self.www_upstream_address.is_some()
            && should_route_to_www(host.as_deref(), self.www_host.as_deref());

        let upstream_address = if route_to_www {
            self.www_upstream_address.as_deref().unwrap()
        } else {
            self.centrale_upstream_address.as_str()
        };

        let peer = HttpPeer::new(upstream_address, false, "localhost".to_string());
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
        // NO CHECKOUT FOR PING
        if !ctx.is_ping {
            // CHECKOUT
            let status = session.response_written().map_or(0, |r| r.status.as_u16());
            let checkout = build_checkout(status, e, ctx);
            self.writer.send_checkout(checkout);
        }
    }
}

fn is_plain_http_request(session: &Session) -> bool {
    session
        .downstream_session
        .server_addr()
        .and_then(|addr| addr.as_inet())
        .map(|addr| addr.port() == 80)
        .unwrap_or(false)
}

fn request_path_and_query(path: &str, query: Option<&str>) -> String {
    let path = if path.is_empty() { "/" } else { path };

    match query {
        Some(query) if !query.is_empty() => format!("{}?{}", path, query),
        _ => path.to_string(),
    }
}

fn build_https_redirect_location(raw_host: Option<&str>, path_and_query: &str) -> Option<String> {
    let host = normalize_redirect_host(raw_host?);
    (!host.is_empty()).then(|| format!("https://{}{}", host, path_and_query))
}

fn normalize_redirect_host(host: &str) -> String {
    let host = host.trim();
    let host = host
        .strip_prefix("http://")
        .or_else(|| host.strip_prefix("https://"))
        .unwrap_or(host);

    let host = host.split('/').next().unwrap_or(host).trim_end_matches('.');
    strip_default_http_port(host)
}

fn strip_default_http_port(host: &str) -> String {
    if host.starts_with('[') {
        if let Some(idx) = host.find(']') {
            let (ip_literal, remainder) = host.split_at(idx + 1);
            if remainder == ":80" {
                return ip_literal.to_string();
            }
        }
        return host.to_string();
    }

    match host.rsplit_once(':') {
        Some((name, port)) if !name.is_empty() && !name.contains(':') && port == "80" => {
            name.to_string()
        }
        _ => host.to_string(),
    }
}

fn request_host(session: &Session) -> Option<String> {
    let req = session.req_header();

    let authority = req.uri.authority().map(|a| a.as_str());
    let host_header = req.headers.get("host").and_then(|v| v.to_str().ok());
    let referer_header = req
        .headers
        .get("referer")
        .or_else(|| req.headers.get("referrer"))
        .and_then(|v| v.to_str().ok());

    resolve_request_host(authority, host_header, referer_header)
}

fn resolve_request_host(
    authority: Option<&str>,
    host_header: Option<&str>,
    referer_header: Option<&str>,
) -> Option<String> {
    let raw = authority
        // HTTP/1.x fallback: Host header
        .or(host_header)
        // Last-resort fallback when Host is unavailable.
        .or(referer_header)?;

    let host = normalize_host(raw);
    (!host.is_empty()).then_some(host)
}

fn is_ip_literal_host(raw_host: &str) -> bool {
    let normalized = normalize_host(raw_host);
    let candidate = if let Some(rest) = normalized.strip_prefix('[') {
        rest.split_once(']')
            .map(|(address, _)| address)
            .unwrap_or(normalized.as_str())
    } else {
        normalized.as_str()
    };

    candidate.parse::<IpAddr>().is_ok()
}

fn normalize_host(host: &str) -> String {
    let host = host.trim();
    let host = host
        .strip_prefix("http://")
        .or_else(|| host.strip_prefix("https://"))
        .unwrap_or(host);

    let host = host.split('/').next().unwrap_or(host);
    let host = host.trim_end_matches('.');

    strip_port(host).to_ascii_lowercase()
}

fn strip_port(host: &str) -> &str {
    if host.starts_with('[') {
        return host;
    }

    match host.rsplit_once(':') {
        Some((name, port))
            if !name.is_empty()
                && !port.is_empty()
                && !name.contains(':')
                && port.chars().all(|ch| ch.is_ascii_digit()) =>
        {
            name
        }
        _ => host,
    }
}

fn should_route_to_www(host: Option<&str>, expected_www_host: Option<&str>) -> bool {
    let Some(host) = host else {
        return false;
    };

    match expected_www_host {
        Some(expected) => host_matches_expected_or_apex(host, expected),
        None => host.starts_with("www."),
    }
}

fn host_matches_expected_or_apex(host: &str, expected: &str) -> bool {
    if host == expected {
        return true;
    }

    if let Some(apex) = expected.strip_prefix("www.") {
        host == apex
    } else {
        host.strip_prefix("www.") == Some(expected)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_https_redirect_location, is_ip_literal_host, normalize_host, request_path_and_query,
        resolve_request_host, should_route_to_www, strip_default_http_port,
    };

    #[test]
    fn ip_literal_hosts_are_rejected_with_or_without_ports() {
        assert!(is_ip_literal_host("127.0.0.1"));
        assert!(is_ip_literal_host("127.0.0.1:443"));
        assert!(is_ip_literal_host("[::1]:443"));
        assert!(is_ip_literal_host("2001:db8::1"));
        assert!(!is_ip_literal_host("app.example.com"));
        assert!(!is_ip_literal_host("127.0.0.1.example.com"));
    }

    #[test]
    fn normalize_host_supports_case_scheme_and_port() {
        assert_eq!(normalize_host("WWW.Example.COM:443"), "www.example.com");
        assert_eq!(
            normalize_host("https://WWW.Example.COM:443/path"),
            "www.example.com"
        );
    }

    #[test]
    fn resolve_request_host_falls_back_to_referer_when_host_is_missing() {
        assert_eq!(
            resolve_request_host(
                None,
                None,
                Some("https://WWW.Example.COM:443/path?utm_source=newsletter"),
            ),
            Some("www.example.com".to_string())
        );
    }

    #[test]
    fn resolve_request_host_prefers_host_over_referer() {
        assert_eq!(
            resolve_request_host(
                None,
                Some("api.example.com"),
                Some("https://www.example.com/path"),
            ),
            Some("api.example.com".to_string())
        );
    }

    #[test]
    fn resolve_request_host_ignores_non_host_referer_values() {
        assert_eq!(
            resolve_request_host(None, None, Some("/docs/getting-started")),
            None
        );
    }

    #[test]
    fn request_path_and_query_handles_empty_and_query_values() {
        assert_eq!(request_path_and_query("", None), "/");
        assert_eq!(
            request_path_and_query("/api/test", Some("a=1&b=2")),
            "/api/test?a=1&b=2"
        );
    }

    #[test]
    fn strip_default_http_port_supports_ipv4_and_ipv6_hosts() {
        assert_eq!(strip_default_http_port("example.com:80"), "example.com");
        assert_eq!(
            strip_default_http_port("example.com:8080"),
            "example.com:8080"
        );
        assert_eq!(strip_default_http_port("[::1]:80"), "[::1]");
        assert_eq!(strip_default_http_port("[::1]:8443"), "[::1]:8443");
    }

    #[test]
    fn build_https_redirect_location_preserves_path_and_query() {
        assert_eq!(
            build_https_redirect_location(Some("http://WWW.Example.COM:80"), "/api/user?x=1"),
            Some("https://WWW.Example.COM/api/user?x=1".to_string())
        );
        assert_eq!(
            build_https_redirect_location(Some("example.com."), "/"),
            Some("https://example.com/".to_string())
        );
        assert_eq!(build_https_redirect_location(None, "/"), None);
    }

    #[test]
    fn should_route_to_www_with_known_domain() {
        assert!(should_route_to_www(
            Some("www.example.com"),
            Some("www.example.com")
        ));
        assert!(should_route_to_www(
            Some("example.com"),
            Some("www.example.com")
        ));
        assert!(!should_route_to_www(
            Some("api.example.com"),
            Some("www.example.com")
        ));
        assert!(!should_route_to_www(
            Some("www.other.com"),
            Some("www.example.com")
        ));
        assert!(!should_route_to_www(
            Some("other.com"),
            Some("www.example.com")
        ));
    }

    #[test]
    fn should_route_to_www_with_prefix_when_domain_is_unknown() {
        assert!(should_route_to_www(Some("www.any-domain.test"), None));
        assert!(!should_route_to_www(Some("api.any-domain.test"), None));
    }
}
