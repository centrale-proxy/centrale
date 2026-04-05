use actix_web::{HttpRequest, HttpResponse, web};
use actix_ws::Message;
use futures_util::{SinkExt, StreamExt};
use reqwest::header::HeaderName;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as TungMessage;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

pub async fn ws_proxy(
    req: HttpRequest,
    stream: web::Payload,
    url: String,
    subdomain: String,
    pass: String,
    role: String,
) -> actix_web::Result<HttpResponse> {
    // 1. Upgrade the client connection
    let (response, client_session, mut client_stream) = actix_ws::handle(&req, stream)?;

    let mut upstream_req = url
        .into_client_request()
        .map_err(|e| actix_web::error::ErrorBadGateway(e))?;

    let headers = upstream_req.headers_mut();

    headers.insert(
        HeaderName::from_static("centrale_subdomain"),
        subdomain.parse().unwrap(),
    );

    headers.insert(
        HeaderName::from_static("centrale_password"),
        pass.parse().unwrap(),
    );

    headers.insert(
        HeaderName::from_static("centrale_role"),
        role.parse().unwrap(),
    );

    // 2. Connect to the upstream WebSocket server
    let (upstream_ws, _) = connect_async(upstream_req)
        .await
        .map_err(|e| actix_web::error::ErrorBadGateway(e))?;

    let (mut upstream_sink, mut upstream_stream) = upstream_ws.split();

    // 3. Spawn a task to forward client → upstream
    let mut session_clone = client_session.clone();
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = client_stream.next().await {
            match msg {
                Message::Text(text) => {
                    if upstream_sink
                        .send(TungMessage::Text(text.to_string().into()))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Message::Binary(bin) => {
                    if upstream_sink
                        .send(TungMessage::Binary(bin.to_vec().into()))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Message::Ping(ping) => {
                    let _ = upstream_sink.send(TungMessage::Ping(ping)).await;
                }
                Message::Close(_) => {
                    let _ = upstream_sink.send(TungMessage::Close(None)).await;
                    break;
                }
                _ => {}
            }
        }
    });

    // 4. Spawn a task to forward upstream → client
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = upstream_stream.next().await {
            match msg {
                TungMessage::Text(text) => {
                    if session_clone.text(text.to_string()).await.is_err() {
                        break;
                    }
                }
                TungMessage::Binary(bin) => {
                    if session_clone.binary(bin).await.is_err() {
                        break;
                    }
                }
                TungMessage::Ping(ping) => {
                    let _ = session_clone.ping(&ping).await;
                }
                TungMessage::Close(_) => {
                    let _ = session_clone.close(None).await;
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}
