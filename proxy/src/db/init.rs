use crate::{
    db::{
        air_token::create_air_token_table, bearer::create_bearer_table,
        cookie::create_cookie_table, subdomain::create_subdomain_table,
        subdomain_user::create_subdomain_user_table, user::create_user_table,
    },
    error::CentraleError,
};
use config::CentraleConfig;
use dir_and_db_pool::db::{DbBool, get_encrypted_connection::get_encrypted_connection};

pub fn init_db(pool: &DbBool) -> Result<(), CentraleError> {
    // USER TABLE
    let db = get_encrypted_connection(pool, CentraleConfig::MASTER_PASSWORD)?;
    // USER TABLE
    create_user_table(&db)?;
    // SUBDOMAIN
    create_subdomain_table(&db)?;
    // SUBDOMAIN_USER
    create_subdomain_user_table(&db)?;
    // bearer
    create_bearer_table(&db)?;
    // AIR TOKEN
    create_air_token_table(&db)?;
    // COOKIE
    create_cookie_table(&db)?;
    Ok(())
}
