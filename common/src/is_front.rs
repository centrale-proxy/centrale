use config::CentraleConfig;

pub fn is_front(url: &str) -> bool {
    let index = format!("{}/", CentraleConfig::get("DESTINATION_SERVER_ADDRESS"));
    if url == index {
        return true;
    }
    if url.starts_with("/src") || url.starts_with("/web") {
        return true;
    }

    // Strip query string and fragment so `/app.js?v=123` is treated as `/app.js`
    let path = url.split_once('?').map(|(p, _)| p).unwrap_or(url);
    let path = path.split_once('#').map(|(p, _)| p).unwrap_or(path);

    // Check common asset extensions (any length)
    const EXTENSIONS: [&str; 12] = [
        ".gif", ".jpg", ".png", ".ttf", ".css", ".map", ".svg", ".ico", ".js", ".json", ".wasm",
        ".woff2",
    ];
    if EXTENSIONS.iter().any(|ext| path.ends_with(ext)) {
        return true;
    }

    if path.starts_with("/favicon") && !path.starts_with("/favicon.ico") {
        return true;
    }
    if path.starts_with("/OpenSans-Regular.") {
        return true;
    }
    false
}
