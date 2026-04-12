use crate::{
    error::CentraleError,
    proxy::{
        authenticate_and_authorize::authenticate_and_authorize,
        get_master_bearer::get_master_bearer_token, is_ws::is_streaming_request,
        proxy_ws::ws_proxy, wildcard::QueryParams,
        ws_authenticate_and_authorize::ws_authenticate_and_authorize,
    },
};
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, web};
use dir_and_db_pool::db::DbBool;
use reqwest::{Method, header};
use std::str::FromStr;

/// Process one wildcard request
pub async fn process_one_request(
    pool: web::Data<DbBool>,
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<QueryParams>,
    //  payload: web::Json<Value>,
) -> Result<HttpResponse, CentraleError> {
    if is_streaming_request(&req) {
        // IS STEAM
        let (_user_id, subdomain, role, pass, url) =
            ws_authenticate_and_authorize(pool, &req, query)?;

        let socket = ws_proxy(req, stream, url, subdomain, pass, role).await?;
        Ok(socket)
    } else {
        // IS NORMAL
        let (_user_id, subdomain, subdomain_user_role, pass, url) =
            authenticate_and_authorize(pool, &req)?;
        // PROXY
        let client = reqwest::Client::new();
        let master_token = get_master_bearer_token()?;
        //let method = req.method();
        let method = Method::from_str(req.method().as_str()).unwrap();
        //println!("method: {:?}", method);
        let request = client
            .request(method.clone(), url)
            .header(header::AUTHORIZATION, format!("Bearer {}", master_token))
            .header("centrale_subdomain", format!("{}", subdomain))
            .header("centrale_password", format!("{}", pass))
            .header("centrale_role", format!("{}", subdomain_user_role));
        // .json(&payload)
        //.send()
        //.await?;
        // if matches!(method, Method::POST | Method::PUT) {
        //   request = request.json(&payload);
        // }

        let response = request.send().await?;

        let status = response.status();
        let body = response.bytes().await?;
        let res = HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body);

        Ok(res)
    }
}
