pub mod create;

use crate::error::CentraleError;
use chrono::Utc;
use config::CentraleConfig;
use rand::{RngCore, rngs::OsRng};
use rusqlite::{Connection, OptionalExtension, Result, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, Serialize)]
pub struct CentraleCookie {
    pub id: Option<i64>,
    pub user_id: i64,
    pub hash: String,
    pub expires: u64,
}

impl CentraleCookie {
    pub fn init_db(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cookie (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id        INTEGER    NOT NULL,
            hash           TEXT       NOT NULL,
            expires        INTEGER    NOT NULL
        )",
            [],
        )?;
        conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_cookie_hash ON cookie (hash)",
            [],
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

    pub fn generate_and_save_client_cookie(
        db: &Connection,
        user_id: i64,
    ) -> Result<String, CentraleError> {
        // 256-bit random token handed to the client.
        let mut raw = [0u8; 32];
        OsRng.fill_bytes(&mut raw);
        let cookie_string = Self::hash_token_bytes(&raw); // hex string the browser stores

        let hash = Self::hash_token(&cookie_string);

        let now = Utc::now().timestamp();
        let timeout = CentraleConfig::get("COOKIE_TIMEOUT").parse::<i64>()? + now;

        db.execute(
            "INSERT INTO cookie (user_id, hash, expires) VALUES (?1, ?2, ?3)",
            params![user_id, hash, timeout],
        )?;

        Ok(cookie_string)
    }

    pub fn validate_client_cookie(
        db: &Connection,
        cookie: &str,
    ) -> Result<Option<i64>, CentraleError> {
        let now = Utc::now().timestamp();
        let hash = Self::hash_token(cookie);

        let user_id = db
            .query_row(
                "SELECT user_id FROM cookie WHERE hash = ?1 AND expires > ?2",
                params![hash, now],
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        Ok(user_id)
    }

    // helper to hex-encode the raw random token
    fn hash_token_bytes(bytes: &[u8]) -> String {
        let mut out = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            out.push_str(&format!("{:02x}", b));
        }
        out
    }

    pub fn new(conn: &Connection, user_id: i64) {
        // DELETE OLD COOKIES
        Self::delete_user_cookies(&conn, user_id).unwrap();
        Self::generate_and_save_client_cookie(&conn, user_id).unwrap();
    }

    pub fn delete_user_cookies(conn: &Connection, user_id: i64) -> Result<()> {
        let _ = conn.execute("DELETE FROM cookie WHERE user_id = ?1", params![user_id])?;
        Ok(())
    }

    pub fn delete_one(&self, conn: &Connection) -> Result<usize> {
        let rows = conn.execute("DELETE FROM cookie WHERE id = ?1 LIMIT 1", params![self.id])?;
        Ok(rows)
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<i64> {
        conn.execute(
            "INSERT INTO cookie (user_id, hash, expired)
         VALUES (?1)",
            params![self.user_id, self.hash, self.expires],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_one(&mut self, conn: &Connection, id: i64) -> Result<Option<CentraleCookie>> {
        let mut stmt =
            conn.prepare("SELECT id, user_id, hash, expires FROM cookie WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(CentraleCookie {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                hash: row.get(2)?,
                expires: row.get(3)?,
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }
}
