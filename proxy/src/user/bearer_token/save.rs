use crate::error::CentraleError;
use common::random::random_numbers;
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;
// use rand::rngs::OsRng;

/// Add bearer_token to db
pub fn save_bearer_token(db: &DbConnection, user_id: i64) -> Result<String, CentraleError> {
    // GENERATE BEARER TOKEN
    let bytes = random_numbers(64);
    let bearer_token = String::from_utf8(bytes)?;
    // INSERT TO DB
    db.execute(
        "INSERT INTO bearer (user_id, bearer) VALUES (?1, ?2)",
        params![user_id, &bearer_token],
    )?;
    Ok(bearer_token)
}
