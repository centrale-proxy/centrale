pub mod process;
pub mod responder;

use crate::error::CentraleError;
use common::random::random_numbers;
use rusqlite::{Connection, OptionalExtension, Result, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, Serialize)]
pub struct CentraleBearer {
    pub id: Option<i64>,
    pub user_id: i64,
    pub hash: String,
    pub expires: u64,
}

impl CentraleBearer {
    pub fn init_db(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS bearer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT,
                user_id INTEGER,
                FOREIGN KEY(user_id) REFERENCES user(id)
            );
            CREATE INDEX IF NOT EXISTS idx_bearer ON bearer (hash);
            ",
        )?;
        Ok(())
    }
    fn hash_token(token: &str) -> String {
        let digest = Sha256::digest(token.as_bytes());
        let mut out = String::with_capacity(64);
        for b in digest {
            out.push_str(&format!("{:02x}", b));
        }
        out
    }

    pub fn generate_and_save_bearer(
        db: &Connection,
        user_id: i64,
    ) -> Result<String, CentraleError> {
        let bytes = random_numbers(64);
        let bearer_token = String::from_utf8(bytes)?;
        let hash = Self::hash_token(&bearer_token);

        db.execute(
            "INSERT INTO bearer (user_id, hash) VALUES (?1, ?2)",
            params![user_id, hash],
        )?;

        Ok(bearer_token)
    }

    pub fn validate_bearer_token(
        db: &Connection,
        bearer: &str,
    ) -> Result<Option<i64>, CentraleError> {
        let hash = Self::hash_token(bearer);

        let user_id = db
            .query_row(
                "SELECT user_id FROM bearer WHERE hash = ?1",
                params![hash],
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        Ok(user_id)
    }

    pub fn new(conn: &Connection, user_id: i64) -> Result<String, CentraleError> {
        // DELETE OLD TOKEN
        Self::delete_user_bearer_tokens(&conn, user_id)?;
        // CREATE NEW TOKEN
        let token = Self::generate_and_save_bearer(&conn, user_id)?;
        Ok(token)
    }

    pub fn delete_user_bearer_tokens(conn: &Connection, user_id: i64) -> Result<()> {
        let _ = conn.execute("DELETE FROM bearer WHERE user_id = ?1", params![user_id])?;
        Ok(())
    }
}
