use crate::error::CentraleError;
use argon2::password_hash::SaltString;
use chrono::Utc;
use config::CentraleConfig;
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;
use rand::rngs::OsRng;

/// Add air_token to db
pub fn save_air_token(db: &DbConnection, user_id: i64) -> Result<String, CentraleError> {
    // DELETE OLD AIR TOKEN
    db.execute("DELETE FROM air_token WHERE user_id = ?1", params![user_id])?;
    // GENERATE AIR TOKEN
    let air_token = SaltString::generate(&mut OsRng);
    let air_token_str = air_token.as_str().to_string();
    // CALCULATE TIMEOUT
    let now = Utc::now();
    let current_unix_epoch = now.timestamp();
    let timeout = CentraleConfig::AIR_TOKEN_TIMEOUT + current_unix_epoch;
    // INSERT TO DB
    db.execute(
        "INSERT INTO air_token (user_id, air_token, timeout) VALUES (?1, ?2, ?3)",
        params![user_id, &air_token.as_str(), timeout],
    )?;
    Ok(air_token_str)
}
