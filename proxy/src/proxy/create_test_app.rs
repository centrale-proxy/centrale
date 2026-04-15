use actix_http::Request;
use actix_web::{
    Error,
    dev::{Service, ServiceResponse},
    web,
};

use crate::user::post::test::_create_test_pool;
use crate::{proxy::create_client::create_client_with_cert, server::routes::routes};
use actix_web::{App, test};

pub async fn _create_test_app() -> impl Service<Request, Response = ServiceResponse, Error = Error>
{
    let db = _create_test_pool();
    let client = create_client_with_cert().unwrap();

    test::init_service(
        App::new()
            .configure(routes)
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(db.clone())),
    )
    .await
}
