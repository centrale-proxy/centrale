pub mod connect_to_port;
//pub mod _payload;

use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use common::convert::string_to_vector;
use common::payload::{CheckIn, WriterPayload};
use futures_util::future::LocalBoxFuture;

//use crate::server::log::convert::string_to_vector;

pub fn log_middleware<S, B>(
    req: ServiceRequest,
    srv: &S,
    log_stream: Arc<Mutex<TcpStream>>,
) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    // --- Log the request ---
    //let req_log = format!("REQ  {} {} {:?}\n", req.method(), req.uri(), req.headers());

    let aaa = CheckIn::new(&req);
    let pl = WriterPayload::CheckIn(aaa);
    let sss = serde_json::to_string(&pl).unwrap();
    let uu = string_to_vector(&sss);

    if let Ok(mut stream) = log_stream.lock() {
        let _ = stream.write_all(&uu);
    }

    let fut = srv.call(req);

    Box::pin(async move {
        let res = fut.await?;
        //let res_log = format!("RES  {:?}\n", res.headers());

        //if let Ok(mut stream) = log_stream.lock() {
        //let _ = stream.write_all(res_log.as_bytes());
        //}

        Ok(res)
    })
}
