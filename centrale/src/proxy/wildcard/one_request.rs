use crate::{
    error::CentraleError,
    proxy::{
        websocket::{is_ws::is_streaming_request, proxy_ws::ws_proxy},
        wildcard::{is_front::is_front, serve_front::serve_front_end},
    },
    server::auth::CentraleUser,
};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use reqwest::{Method, header};
use std::str::FromStr;

/// Process one wildcard request
pub async fn process_one_request(
    req: HttpRequest,
    stream: web::Payload,
    client: web::Data<reqwest::Client>,
    user: CentraleUser,
) -> Result<HttpResponse, CentraleError> {
    if is_streaming_request(&req) {
        // IS STREAM
        let socket = ws_proxy(req, stream, user.url, user.subdomain, user.pass, user.role).await?;
        Ok(socket)
    } else {
        // IS FRONT
        if is_front(&user.url) {
            // serve front
            let front = serve_front_end(req).await;
            return Ok(front);
        }
        // IS NOT FRONT END
        let method = Method::from_str(req.method().as_str());

        let unwrapped_method = match method {
            Ok(method) => method,
            Err(_err) => return Err(CentraleError::InvalidMethod),
        };

        let url_local = format!("https://{}", user.url);

        let request = client
            .request(unwrapped_method.clone(), url_local)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user.destination_bearer),
            )
            .header("centrale_subdomain", format!("{}", user.subdomain))
            .header("centrale_password", format!("{}", user.pass))
            .header("centrale_role", format!("{}", user.role));

        let response = request.send().await;
        let response = response?;
        let status = response.status();
        let body = response.bytes().await?;

        let res = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body);

        Ok(res)
    }
}
