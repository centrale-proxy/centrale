/// Strip a port (and IPv6 brackets) from a Host header value.
pub fn host_only(host: &str) -> &str {
    let host = host.trim();
    if let Some(rest) = host.strip_prefix('[') {
        // IPv6 literal, e.g. [::1]:8080
        return rest.split(']').next().unwrap_or(rest);
    }
    host.split(':').next().unwrap_or(host)
}

/// Pull the hostname out of a Referer URL: scheme://[user@]host[:port]/path?...
pub fn host_from_referrer(referrer: &str) -> Option<String> {
    let after_scheme = referrer.split("://").nth(1).unwrap_or(referrer);
    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or(after_scheme);
    let authority = authority.rsplit('@').next().unwrap_or(authority); // drop userinfo
    let hostname = host_only(authority).trim();
    if hostname.is_empty() {
        None
    } else {
        Some(hostname.to_string())
    }
}

/// Subdomain portion of a hostname, or None if there isn't one.
/// "app.example.com" -> Some("app"), "example.com" -> None,
/// "a.b.example.co.uk" -> Some("a.b").
pub fn extract_subdomain(hostname: &str) -> Option<String> {
    let host = hostname.trim().trim_end_matches('.').to_ascii_lowercase();
    if host.is_empty() || host.parse::<std::net::IpAddr>().is_ok() {
        return None; // empty or a bare IP has no subdomain
    }

    let labels: Vec<&str> = host.split('.').filter(|l| !l.is_empty()).collect();
    let registrable_labels = public_suffix_labels(&labels) + 1;

    if labels.len() <= registrable_labels {
        return None; // e.g. example.com / co.uk — nothing to the left
    }

    Some(labels[..labels.len() - registrable_labels].join("."))
}

/// Minimal heuristic for multi-label public suffixes. For full ccTLD correctness,
/// replace this with the `psl` or `publicsuffix` crate.
pub fn public_suffix_labels(labels: &[&str]) -> usize {
    const TWO_LABEL_SUFFIXES: &[&str] = &[
        "co.uk", "org.uk", "gov.uk", "ac.uk", "me.uk", "com.au", "net.au", "org.au", "gov.au",
        "co.jp", "co.nz", "co.za", "co.in", "co.kr", "com.br", "com.mx", "com.cn", "com.sg",
        "com.tr",
    ];

    if labels.len() >= 2 {
        let last_two = format!("{}.{}", labels[labels.len() - 2], labels[labels.len() - 1]);
        if TWO_LABEL_SUFFIXES.contains(&last_two.as_str()) {
            return 2;
        }
    }
    1
}
