use actix_web::HttpRequest;

pub fn is_streaming_request(req: &HttpRequest) -> bool {
    // Check for WebSocket upgrade
    let is_websocket = {
        let is_upgrade = req
            .headers()
            .get("upgrade")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_lowercase().contains("websocket"))
            .unwrap_or(false);

        let is_connection_upgrade = req
            .headers()
            .get("connection")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_lowercase().contains("upgrade"))
            .unwrap_or(false);

        is_upgrade && is_connection_upgrade
    };

    // 1. Transfer-Encoding: chunked
    let is_chunked = req
        .headers()
        .get("transfer-encoding")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_lowercase().contains("chunked"))
        .unwrap_or(false);

    // 2. No Content-Length (body size unknown upfront)
    let no_content_length = req.headers().get("content-length").is_none();

    // 3. Explicit content-type hints (SSE, octet-stream, etc.)
    let streaming_content_type = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|ct| {
            ct.contains("text/event-stream")
                || ct.contains("application/octet-stream")
                || ct.contains("multipart/")
        })
        .unwrap_or(false);

    is_websocket || is_chunked || (no_content_length && streaming_content_type)
}
