use common::client_ip::ClientIP;
use pingora::proxy::Session;

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
