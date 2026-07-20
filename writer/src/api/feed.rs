use actix_web::{HttpResponse, web};
use futures::stream::{self, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

pub async fn feed(feed_tx: web::Data<tokio::sync::broadcast::Sender<String>>) -> HttpResponse {
    let initial =
        stream::once(async { Ok::<_, actix_web::Error>(web::Bytes::from("data: {}\n\n")) });

    let broadcast = BroadcastStream::new(feed_tx.subscribe())
        .filter(|message| futures::future::ready(message.is_ok()))
        .map(|message| {
            let data = message.unwrap();
            Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: {data}\n\n")))
        });

    let stream = initial.chain(broadcast);

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("X-Accel-Buffering", "no"))
        .streaming(stream)
}
