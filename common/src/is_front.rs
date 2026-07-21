use config::CentraleConfig;

pub fn is_front(url: &str) -> bool {
    let index = format!("{}/", CentraleConfig::get("DESTINATION_SERVER_ADDRESS"));

    if url == index {
        return true;
    }

    if url.starts_with("/src") || url.starts_with("/web") {
        return true;
    }
    // Check common asset extensions (any length)
    const EXTENSIONS: [&str; 12] = [
        ".gif", ".jpg", ".png", ".ttf", ".css", ".map", ".svg", ".ico", ".js", ".json", ".wasm",
        ".woff2",
    ];
    if EXTENSIONS.iter().any(|ext| url.ends_with(ext)) {
        return true;
    }
    if url.starts_with("/favicon") && !url.starts_with("/favicon.ico") {
        return true;
    }
    if url.starts_with("/OpenSans-Regular.") {
        return true;
    }
    false
}
