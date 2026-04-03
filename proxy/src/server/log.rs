pub mod connect_to_port;

use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use common::convert::string_to_vector;
use common::payload::{CheckIn, WriterPayload};
use futures_util::future::LocalBoxFuture;
use mio::net::UdpSocket;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub fn log_middleware<S, B>(
    req: ServiceRequest,
    srv: &S,
    socket: Arc<Mutex<UdpSocket>>,
    addr: SocketAddr,
) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    let check_in = CheckIn::new(&req);
    actix_web::rt::spawn(async move {
        let pl = WriterPayload::CheckIn(check_in);
        let sss = serde_json::to_string(&pl).unwrap();
        let uu = string_to_vector(&sss);

        if let Ok(stream) = socket.lock() {
            match stream.send_to(&uu, addr) {
                Ok(_) => {
                    // println!("sent");
                }
                Err(err) => {
                    eprintln!("Unable to write to stream: {}", err);
                }
            }
        }
    });

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
