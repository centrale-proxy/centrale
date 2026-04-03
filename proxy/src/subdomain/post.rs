use crate::error::CentraleError;
use argon2::password_hash::SaltString;
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;
use rand::rngs::OsRng;

pub fn post_subdomain(
    db: &DbConnection,
    subdomain: &String,
    user_id: i64,
) -> Result<String, CentraleError> {
    //
    let mut stmt = db.prepare(&"SELECT COUNT(*) FROM subdomain WHERE subdomain = ?1")?;
    let count: i64 = stmt.query_row(params![subdomain], |row| row.get(0))?;

    if count > 0 {
        // USERS(s) EXIST. CANNOT HAVE MORE
        return Err(CentraleError::SuchSubdomainExists);
    } else {
        let password = SaltString::generate(&mut OsRng);
        db.execute(
            "INSERT INTO subdomain (subdomain, password, user_id) VALUES (?1, ?2, ?3)",
            params![subdomain, password.as_str(), user_id],
        )?;

        db.execute(
            "INSERT INTO subdomain_user (subdomain, user_id, role) VALUES (?1, ?2, ?3)",
            params![subdomain, password.as_str(), "admin".to_string()],
        )?;
        // TBD INSERT META

        Ok(subdomain.clone())
    }
}
