use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use futures_util::future::LocalBoxFuture;

pub fn log_middleware<S, B>(
    req: ServiceRequest,
    srv: &S,
) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    // TBD STREAM TO LOG
    println!("{:?}", req.headers());

    let fut = srv.call(req);

    Box::pin(async move {
        let res = fut.await?;
        // TBD STREAM TO LOG
        println!("{:?}", res.headers());
        Ok(res)
    })
}
