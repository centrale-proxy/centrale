use crate::{
    db::{
        bearer::create_bearer_table, cookie::create_cookie_table,
        subdomain::create_subdomain_table, subdomain_user::create_subdomain_user_table,
        user::create_user_table,
    },
    error::CentraleError,
};
use dir_and_db_pool::db::DbBool;

pub fn init_db(pool: &DbBool) -> Result<(), CentraleError> {
    // USER TABLE
    let db = pool.get()?;
    // USER TABLE
    create_user_table(&db)?;
    // SUBDOMAIN
    create_subdomain_table(&db)?;
    // SUBDOMAIN_USER
    create_subdomain_user_table(&db)?;
    // bearer
    create_bearer_table(&db)?;
    // COOKIE
    create_cookie_table(&db)?;
    Ok(())
}
