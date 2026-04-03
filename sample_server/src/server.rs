use crate::{
    auth::auth_MASTER_BEARER_TOKEN, db::get_sample_db, error::SampleServerError, routes::routes,
};
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;

#[actix_web::main]
pub async fn start_server() -> Result<(), SampleServerError> {
    let db = get_sample_db()?;

    println!(
        "server started at {}",
        CentraleConfig::SAMPLE_SERVER_ADDRESS
    );

    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(from_fn(auth_MASTER_BEARER_TOKEN))
            .app_data(web::Data::new(db.clone()))
    })
    .bind(CentraleConfig::SAMPLE_SERVER_ADDRESS)?
    .run()
    .await?;

    Ok(())
}
