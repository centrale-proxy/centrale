use crate::{
    auth::auth_master_bearer_token, db::get_sample_db, error::SampleServerError, routes::routes,
};
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;

use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

pub struct DbPoolRegistry {
    pub pools: HashMap<String, DbBool>,
}

impl DbPoolRegistry {
    pub fn get(&self, key: &str) -> Option<&DbBool> {
        self.pools.get(key)
    }
}

#[actix_web::main]
pub async fn start_server() -> Result<(), SampleServerError> {
    let pools = HashMap::new();
    // pools.insert("primary".to_string(), get_sample_db()?);
    // pools.insert("analytics".to_string(), get_analytics_db()?);

    let registry = Arc::new(RwLock::new(DbPoolRegistry { pools }));

    // let db = get_sample_db()?;

    println!(
        "server started at {}",
        CentraleConfig::SAMPLE_SERVER_ADDRESS
    );

    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(from_fn(auth_master_bearer_token))
            // .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(registry.clone()))
    })
    .bind(CentraleConfig::SAMPLE_SERVER_ADDRESS)?
    .run()
    .await?;

    Ok(())
}
