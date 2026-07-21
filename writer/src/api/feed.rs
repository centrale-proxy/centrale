use crate::db::get_last_entries;
use actix_web::{HttpResponse, web};
use futures::stream::{self, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

pub async fn feed(
    feed_tx: web::Data<tokio::sync::broadcast::Sender<String>>,
    db_pool: web::Data<dir_and_db_pool::db::DbPool>,
) -> HttpResponse {
    // Fetch last entries for the initial payload
    let initial_entries = db_pool
        .get()
        .ok()
        .and_then(|db_pool| get_last_entries(&db_pool, 300).ok())
        .unwrap_or_default();

    // One stream item (= one flushed chunk) per entry
    let initial = stream::iter(initial_entries.into_iter().filter_map(|entry| {
        serde_json::to_string(&entry)
            .ok()
            .map(|json| Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: {json}\n\n"))))
    }));

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
