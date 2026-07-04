use crate::{connect_to_writer::WriterClient, request::client_ip, response::build_checkout};
use async_trait::async_trait;
use bytes::Bytes;
use common::payload::CheckIn;
use pingora::prelude::{HttpPeer, ProxyHttp, Result, Session};
use uuid::Uuid;

pub struct LoadBalancer {
    pub centrale_upstream_address: String,
    pub www_upstream_address: Option<String>,
    pub www_host: Option<String>,
    pub writer: WriterClient,
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
        let request_bytes = session.downstream_session.to_h1_raw().to_vec();
        let ip = client_ip(session);
        let checkin = CheckIn::new(ip, request_bytes, ctx.x_id.clone());
        self.writer.send_checkin(checkin);
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
        let status = session.response_written().map_or(0, |r| r.status.as_u16());

        let checkout = build_checkout(status, e, ctx);
        self.writer.send_checkout(checkout);
    }
}

fn request_host(session: &Session) -> Option<String> {
    let req = session.req_header();

    let raw = req
        .uri
        .authority()
        .map(|a| a.as_str())
        // HTTP/1.x fallback: Host header
        .or_else(|| req.headers.get("host").and_then(|v| v.to_str().ok()))?;

    let host = normalize_host(raw);
    (!host.is_empty()).then_some(host)
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
    use super::{normalize_host, should_route_to_www};

    #[test]
    fn normalize_host_supports_case_scheme_and_port() {
        assert_eq!(normalize_host("WWW.Example.COM:443"), "www.example.com");
        assert_eq!(
            normalize_host("https://WWW.Example.COM:443/path"),
            "www.example.com"
        );
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
