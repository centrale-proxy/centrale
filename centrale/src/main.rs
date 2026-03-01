use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn wildcard_handler(req: HttpRequest) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/{_:.*}", web::get().to(wildcard_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
