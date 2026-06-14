pub fn is_front(url: &str) -> bool {
    // index
    if url == "/" {
        return true;
    }

    // path prefixes
    if url.starts_with("/src") || url.starts_with("/web") {
        return true;
    }

    // asset extensions (all 4 chars, so ends_with is equivalent to the JS slice)
    const EXTENSIONS: [&str; 8] = [
        ".gif", ".jpg", ".png", ".ttf", ".css", ".map", ".svg", ".ico",
    ];
    if EXTENSIONS.iter().any(|ext| url.ends_with(ext)) {
        return true;
    }

    // /favicon* but not /favicon.ico
    if url.starts_with("/favicon") && !url.starts_with("/favicon.ico") {
        return true;
    }

    // OpenSans font
    if url.starts_with("/OpenSans-Regular.") {
        return true;
    }

    false
}
