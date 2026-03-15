use actix_http::Request;
use actix_web::{
    Error,
    dev::{Service, ServiceResponse},
    web,
};

use crate::routes::routes;
use crate::user::register::_create_test_pool;
use actix_web::{App, test};

pub async fn _create_test_app() -> impl Service<Request, Response = ServiceResponse, Error = Error>
{
    let db = _create_test_pool();
    test::init_service(
        App::new()
            .configure(routes)
            .app_data(web::Data::new(db.clone())),
    )
    .await
}
