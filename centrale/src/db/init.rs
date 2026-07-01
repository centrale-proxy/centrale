use crate::{
    api::user::{bearer_token::CentraleBearer, cookie::CentraleCookie},
    db::{
        get_db::get_centrale_db, subdomain::create_subdomain_table,
        subdomain_user::create_subdomain_user_table, user::create_user_table,
    },
    error::CentraleError,
};
use dir_and_db_pool::db::DbPool;

pub fn init_db(pool: &DbPool) -> Result<(), CentraleError> {
    // USER TABLE
    let db = get_centrale_db(pool)?;
    // USER TABLE
    create_user_table(&db)?;
    // SUBDOMAIN
    create_subdomain_table(&db)?;
    // SUBDOMAIN_USER
    create_subdomain_user_table(&db)?;
    // BEARER
    CentraleBearer::init_db(&db)?;
    // COOKIE
    CentraleCookie::init_db(&db)?;

    Ok(())
}
