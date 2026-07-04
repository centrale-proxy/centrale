use pingora::proxy::Session;

pub fn client_for_logging(session: &Session) -> String {
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
