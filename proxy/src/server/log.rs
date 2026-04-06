use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use common::convert::string_to_vector;
use common::payload::{CheckIn, CheckOut, WriterPayload};
use futures_util::future::LocalBoxFuture;
use log::error;
use mio::net::UdpSocket;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub fn send_payload(socket: Arc<Mutex<UdpSocket>>, addr: SocketAddr, payload: WriterPayload) {
    let payload_string = serde_json::to_string(&payload).unwrap();
    let payload_vector = string_to_vector(&payload_string);
    if let Ok(stream) = socket.lock() {
        if let Err(err) = stream.send_to(&payload_vector, addr) {
            error!("Unable to write to stream: {}", err);
        }
    }
}

pub fn log_middleware<S, B>(
    req: ServiceRequest,
    srv: &S,
    socket: Arc<Mutex<UdpSocket>>,
    addr: SocketAddr,
) -> LocalBoxFuture<'static, Result<ServiceResponse<BoxBody>, Error>>
// <-- BoxBody here
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    let check_in = CheckIn::new(&req);
    let soc_1 = socket.clone();
    let soc_2 = socket.clone();
    actix_web::rt::spawn(async move {
        let check_in_payload = WriterPayload::CheckIn(check_in);
        send_payload(soc_1, addr, check_in_payload);
    });

    let fut = srv.call(req);

    Box::pin(async move {
        let res = fut.await?;
        let status = res.status().as_u16();
        let is_ok = status == 200;
        let is_also_ok = status == 304;
        if is_ok || is_also_ok {
            let check_out = CheckOut::new(res.status(), None);
            let check_out_payload = WriterPayload::CheckOut(check_out);
            send_payload(soc_2, addr, check_out_payload);
            // println!("res_log {:?}", check_out);
            Ok(res.map_into_boxed_body())
        } else {
            let headers = res.headers().clone();
            let status_code = res.status();
            let (req, response) = res.into_parts();
            let body_bytes = actix_web::body::to_bytes(response.into_body())
                .await
                .unwrap_or_default();

            let check_out = CheckOut::new(status_code, Some(&body_bytes));
            println!("res_log {:?}", &check_out);
            let check_out_payload = WriterPayload::CheckOut(check_out);
            send_payload(soc_2, addr, check_out_payload);
            let mut new_res = HttpResponse::build(status_code);
            for (key, val) in headers.iter() {
                new_res.append_header((key, val));
            }
            let res = ServiceResponse::new(req, new_res.body(body_bytes));
            Ok(res)
        }
    })
}
