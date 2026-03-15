use crate::error::CentraleError;
use argon2::password_hash::SaltString;
use chrono::Utc;
use config::CentraleConfig;
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;
use rand::rngs::OsRng;

/// Add cookie to db
pub fn save_cookie(db: &DbConnection, user_id: i64) -> Result<String, CentraleError> {
    // DELETE OLD COOKIE
    db.execute("DELETE FROM cookie WHERE user_id = ?1", params![user_id])?;
    // GENERATE COOKIE
    let cookie = SaltString::generate(&mut OsRng);
    let cookie_str = cookie.as_str().to_string();
    // CALCULATE TIMEOUT
    let now = Utc::now();
    let current_unix_epoch = now.timestamp();
    let timeout = CentraleConfig::COOKIE_TIMEOUT + current_unix_epoch;
    // INSERT TO DB
    db.execute(
        "INSERT INTO cookie (user_id, cookie, timeout) VALUES (?1, ?2, ?3)",
        params![user_id, &cookie.as_str(), timeout],
    )?;
    Ok(cookie_str)
}
