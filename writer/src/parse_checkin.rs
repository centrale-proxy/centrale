use std::collections::HashMap;

use common::{names::RandomName, payload::CheckIn};
use serde_derive::{Deserialize, Serialize};

use crate::subdomain::{extract_subdomain, host_from_referrer, host_only};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsedCheckIn {
    pub url: Option<String>,
    pub query: Option<String>,
    pub ua: Option<String>, // STRING
    pub method: Option<String>,
    pub referrer: Option<String>,
    pub host: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,  // PARSED UA
    pub is_bot: bool,             // GOOGLE OR FB CRAWLER
    pub lead: Option<String>,     // GOOGLE BING OR FB
    pub campaign: Option<String>, // utm_campaign
    pub anon_name: String,
    pub subdomain: String,
}

impl ParsedCheckIn {
    pub fn parse_checkin(payload: &CheckIn, names: &mut HashMap<String, String>) -> ParsedCheckIn {
        let text = String::from_utf8_lossy(&payload.bytes);
        let ip = payload.ip.for_logging();
        let ip_only = ip.split(':').next().unwrap_or(&ip).to_string();
        let anon_name = names
            .entry(ip_only)
            .or_insert_with(|| RandomName::new().name)
            .clone();

        Self::parse_checkin_text(text.as_ref(), anon_name.clone(), &payload.host)
    }
    pub fn parse_checkin_text(
        text: &str,
        anon_name: String,
        host: &Option<String>,
    ) -> ParsedCheckIn {
        // Normalize line endings: handle \r\n, \n, AND lone \r. str::lines() does not
        // split on a bare carriage return, which silently collapses such requests.
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");

        let request_line = normalized.lines().next().unwrap_or_default();
        let (method, url, query) = parse_request_line(request_line);

        let mut ua: Option<String> = None;
        let mut referrer: Option<String> = None;
        // let mut host: Option<String> = None;

        for line in normalized.lines().skip(1) {
            if line.is_empty() {
                break; // end of headers, start of body
            }

            // HTTP/2 carries the host as the ":authority" pseudo-header. Strip a single
            // leading ':' so split_once yields a usable name instead of an empty one.
            let line = line.strip_prefix(':').unwrap_or(line);

            let Some((header_name, header_value)) = line.split_once(':') else {
                continue;
            };

            let name = header_name.trim().to_ascii_lowercase();
            let value = header_value.trim().to_string();

            match name.as_str() {
                "user-agent" if ua.is_none() => ua = Some(value),
                "referer" | "referrer" if referrer.is_none() => referrer = Some(value),
                // "host" | "authority" if host.is_none() => host = Some(value),
                _ => {}
            }
        }

        let os = ua.as_deref().and_then(infer_os);
        let browser = ua.as_deref().and_then(infer_browser);
        let is_bot = ua.as_deref().map(is_bot_user_agent).unwrap_or(false);

        let lead = query
            .as_deref()
            .and_then(|q| query_value(q, "utm_source"))
            .or_else(|| referrer.as_deref().and_then(lead_from_referrer));

        let campaign = query
            .as_deref()
            .and_then(|q| query_value(q, "utm_campaign"));

        // Subdomain from Host, falling back to the Referer's hostname.
        let subdomain = {
            let host_hostname = host.as_deref().map(|h| host_only(h).to_string());
            let referrer_hostname = referrer.as_deref().and_then(host_from_referrer);

            host_hostname
                .as_deref()
                .and_then(extract_subdomain)
                .or_else(|| referrer_hostname.as_deref().and_then(extract_subdomain))
                .unwrap_or_default()
        };

        ParsedCheckIn {
            url,
            query,
            ua,
            method,
            referrer,
            host: host.clone(),
            os,
            browser,
            is_bot,
            lead,
            campaign,
            anon_name,
            subdomain,
        }
    }
}

fn parse_request_line(line: &str) -> (Option<String>, Option<String>, Option<String>) {
    let mut parts = line.split_whitespace();

    let method = parts.next().map(|v| v.to_string());
    let target = parts.next().unwrap_or_default();

    if target.is_empty() {
        return (method, None, None);
    }

    let (url, query) = split_target_and_query(target);
    (method, url, query)
}

fn split_target_and_query(target: &str) -> (Option<String>, Option<String>) {
    match target.split_once('?') {
        Some((path, raw_query)) => {
            let path_value = if path.is_empty() {
                None
            } else {
                Some(path.to_string())
            };

            let query_value = if raw_query.is_empty() {
                None
            } else {
                Some(raw_query.to_string())
            };

            (path_value, query_value)
        }
        None => (Some(target.to_string()), None),
    }
}

fn query_value(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (k, v) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => (pair, ""),
        };

        if k.eq_ignore_ascii_case(key) {
            if v.is_empty() {
                return None;
            }

            return Some(v.to_string());
        }
    }

    None
}

fn infer_os(ua: &str) -> Option<String> {
    let lower = ua.to_ascii_lowercase();

    if lower.contains("windows") {
        Some("Windows".to_string())
    } else if lower.contains("android") {
        Some("Android".to_string())
    } else if lower.contains("iphone") || lower.contains("ipad") || lower.contains("ios") {
        Some("iOS".to_string())
    } else if lower.contains("mac os") || lower.contains("macintosh") {
        Some("macOS".to_string())
    } else if lower.contains("linux") {
        Some("Linux".to_string())
    } else {
        None
    }
}

fn infer_browser(ua: &str) -> Option<String> {
    let lower = ua.to_ascii_lowercase();

    if lower.contains("edg/") {
        Some("Edge".to_string())
    } else if lower.contains("opr/") || lower.contains("opera") {
        Some("Opera".to_string())
    } else if lower.contains("chrome/") {
        Some("Chrome".to_string())
    } else if lower.contains("firefox/") {
        Some("Firefox".to_string())
    } else if lower.contains("safari/") && !lower.contains("chrome/") {
        Some("Safari".to_string())
    } else if lower.contains("trident/") || lower.contains("msie") {
        Some("Internet Explorer".to_string())
    } else if lower.contains("curl/") {
        Some("curl".to_string())
    } else {
        None
    }
}

fn is_bot_user_agent(ua: &str) -> bool {
    let lower = ua.to_ascii_lowercase();

    lower.contains("googlebot")
        || lower.contains("bingbot")
        || lower.contains("facebookexternalhit")
        || lower.contains("crawler")
        || lower.contains("spider")
        || lower.contains("bot")
}

fn lead_from_referrer(referrer: &str) -> Option<String> {
    let lower = referrer.to_ascii_lowercase();

    if lower.contains("google.") {
        Some("google".to_string())
    } else if lower.contains("bing.") {
        Some("bing".to_string())
    } else if lower.contains("facebook.") || lower.contains("fb.") {
        Some("facebook".to_string())
    } else if lower.contains("duckduckgo.") {
        Some("duckduckgo".to_string())
    } else if lower.contains("yahoo.") {
        Some("yahoo".to_string())
    } else if lower.contains("t.co") || lower.contains("twitter.") {
        Some("twitter".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::parse_checkin::ParsedCheckIn;

    #[test]
    fn parses_checkin2_text_into_parsed_checkin() {
        let text = "GET /hello/world?utm_source=google&utm_campaign=spring-sale HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36\r\nReferer: https://www.google.com/search?q=hello\r\n\r\n";

        let parsed = ParsedCheckIn::parse_checkin_text(text, "test".to_string(), &None);

        //  assert_eq!(parsed.checkin, 123);
        // assert_eq!(parsed.ip.as_deref(), Some("1.2.3.4"));
        assert_eq!(parsed.method.as_deref(), Some("GET"));
        assert_eq!(parsed.url.as_deref(), Some("/hello/world"));
        assert_eq!(
            parsed.query.as_deref(),
            Some("utm_source=google&utm_campaign=spring-sale")
        );
        assert_eq!(parsed.host.as_deref(), Some("example.com"));
        assert_eq!(parsed.browser.as_deref(), Some("Chrome"));
        assert_eq!(parsed.os.as_deref(), Some("macOS"));
        assert!(!parsed.is_bot);
        assert_eq!(parsed.lead.as_deref(), Some("google"));
        assert_eq!(parsed.campaign.as_deref(), Some("spring-sale"));
    }

    #[test]
    fn marks_bot_user_agents() {
        let text = "GET /robots.txt HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Googlebot/2.1 (+http://www.google.com/bot.html)\r\n\r\n";
        let parsed = ParsedCheckIn::parse_checkin_text(text, "test".to_string(), &None);

        assert_eq!(parsed.method.as_deref(), Some("GET"));
        assert_eq!(parsed.url.as_deref(), Some("/robots.txt"));
        assert!(parsed.is_bot);
        assert_eq!(parsed.lead, None);
    }
}
