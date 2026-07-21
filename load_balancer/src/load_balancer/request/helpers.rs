use common::client_ip::ClientIP;
use pingora::{
    http::ResponseHeader,
    prelude::{Result, Session},
};
use std::net::IpAddr;

pub fn client_ip(session: &Session) -> ClientIP {
    let headers = &session.req_header().headers;

    let forwarded = {
        let values: Vec<String> = headers
            .get_all("forwarded")
            .iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(extract_all_forwarded_for)
            .collect();
        if values.is_empty() {
            None
        } else {
            Some(values.join(", "))
        }
    };

    let x_forwarded_for = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    let x_real_ip = headers
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    let client_addr = session.client_addr().map(|addr| addr.to_string());

    ClientIP {
        forwarded,
        x_forwarded_for,
        x_real_ip,
        client_addr,
    }
}

fn extract_all_forwarded_for(forwarded_header: &str) -> Vec<String> {
    let mut result = Vec::new();
    for element in forwarded_header.split(',') {
        for pair in element.split(';') {
            let Some((key, value)) = pair.split_once('=') else {
                continue;
            };
            if !key.trim().eq_ignore_ascii_case("for") {
                continue;
            }
            let value = value.trim();
            let value = value
                .strip_prefix('"')
                .and_then(|inner| inner.strip_suffix('"'))
                .unwrap_or(value);
            if !value.is_empty() {
                result.push(value.to_string());
            }
        }
    }
    result
}

pub fn request_host_and_path(session: &Session) -> (Option<String>, String) {
    let req = session.req_header();
    let host = req
        .uri
        .authority()
        .map(|authority| authority.as_str().to_string())
        .or_else(|| {
            req.headers
                .get("host")
                .and_then(|value| value.to_str().ok())
                .map(str::to_string)
        });
    let path_and_query = request_path_and_query(req.uri.path(), req.uri.query());

    (host, path_and_query)
}

pub async fn reject_ip_literal_host(session: &mut Session, host: Option<&str>) -> Result<bool> {
    if !host.is_some_and(is_ip_literal_host) {
        return Ok(false);
    }

    let mut response = ResponseHeader::build(421, Some(3))?;
    response.insert_header("Content-Length", "0")?;
    response.insert_header("Cache-Control", "no-store")?;
    session
        .write_response_header(Box::new(response), true)
        .await?;

    Ok(true)
}

pub async fn redirect_http_to_https(
    session: &mut Session,
    force_https_redirect: bool,
    host: Option<&str>,
    path_and_query: &str,
) -> Result<bool> {
    if !force_https_redirect || !is_plain_http_request(session) {
        return Ok(false);
    }

    let Some(location) = build_https_redirect_location(host, path_and_query) else {
        return Ok(false);
    };

    let mut response = ResponseHeader::build(308, Some(3))?;
    response.insert_header("Location", location)?;
    response.insert_header("Content-Length", "0")?;
    response.insert_header("Cache-Control", "no-store")?;
    session
        .write_response_header(Box::new(response), true)
        .await?;

    Ok(true)
}

fn is_plain_http_request(session: &Session) -> bool {
    session
        .downstream_session
        .server_addr()
        .and_then(|addr| addr.as_inet())
        .map(|addr| addr.port() == 80)
        .unwrap_or(false)
}

pub fn request_path_and_query(path: &str, query: Option<&str>) -> String {
    let path = if path.is_empty() { "/" } else { path };

    match query {
        Some(query) if !query.is_empty() => format!("{}?{}", path, query),
        _ => path.to_string(),
    }
}

pub fn build_https_redirect_location(
    raw_host: Option<&str>,
    path_and_query: &str,
) -> Option<String> {
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

pub fn request_host(session: &Session) -> Option<String> {
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

pub(super) fn is_ip_literal_host(raw_host: &str) -> bool {
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

pub fn should_route_to_www(host: Option<&str>, expected_www_host: Option<&str>) -> bool {
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
