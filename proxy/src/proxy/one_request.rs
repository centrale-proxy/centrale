use crate::{
    error::CentraleError,
    proxy::{
        authenticate_and_authorize::authenticate_and_authorize,
        create_client::create_client_with_cert, is_ws::is_streaming_request, proxy_ws::ws_proxy,
        wildcard::QueryParams, ws_authenticate_and_authorize::ws_authenticate_and_authorize,
    },
};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use reqwest::{Method, header};
use std::str::FromStr;

/// Process one wildcard request
pub async fn process_one_request(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<QueryParams>,
    client: web::Data<reqwest::Client>,
) -> Result<HttpResponse, CentraleError> {
    if is_streaming_request(&req) {
        println!("new stream");
        // IS STEAM
        let (_user_id, subdomain, role, pass, url) =
            ws_authenticate_and_authorize(pool, &req, query)?;
        println!("new stream url {}", {:?});
        let socket = ws_proxy(req, stream, url, subdomain, pass, role).await?;
        Ok(socket)
    } else {
        // IS HTTPS REQUEST
        let (_user_id, subdomain, subdomain_user_role, pass, url) =
            authenticate_and_authorize(pool, &req)?;

        // let client = create_client_with_cert()?;

        let master_token = CentraleConfig::master_bearer_token();
        let method = Method::from_str(req.method().as_str());

        let unwrapped_method = match method {
            Ok(method) => method,
            Err(_err) => return Err(CentraleError::InvalidMethod),
        };

        let url_local = format!("https://{}", url);
        //
        let request = client
            .request(unwrapped_method.clone(), url_local)
            .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
            .header("centrale_subdomain", format!("{}", subdomain))
            .header("centrale_password", format!("{}", pass))
            .header("centrale_role", format!("{}", subdomain_user_role));

        let response = request.send().await;
        let response = response?;
        let status = response.status();
        let body = response.bytes().await?;

        let res = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body);

        Ok(res)
    }
}
