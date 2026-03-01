use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn handle_request(req: HttpRequest) -> impl Responder {
    let host = req.headers().get("Host");
    let referer = req.headers().get("Referer");

    if host.is_some() {
        "OK"
    } else if referer.is_some() {
        // CLOUDFLARE HAS NO HOST, ONLY REFERRER
        "OK"
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({ "error": "No authentication" }));
        "No"
    }
}
