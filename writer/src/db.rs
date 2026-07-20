use crate::{error::WriterError, parse_checkin::ParsedCheckIn};
use common::payload::{CheckIn, CheckOut};
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;

pub fn init_writer_db(conn: &DbConnection) -> Result<(), WriterError> {
    // Create the check_ins table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS writer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            x_id TEXT NOT NULL UNIQUE,
            forwarded TEXT,
            x_forwarded_for TEXT,
            x_real_ip TEXT,
            client_addr TEXT,
            client_ip TEXT,
            client_port INTEGER,
            url TEXT,
            query TEXT,
            ua TEXT,
            method TEXT,
            referrer TEXT,
            host TEXT,
            os TEXT,
            browser TEXT,
            is_bot BOOLEAN NOT NULL DEFAULT 0,
            lead TEXT,
            campaign TEXT,
            checkin INTEGER NOT NULL,
            checkout INTEGER,
            error TEXT,
            status INTEGER,
            anon_name TEXT,
            timer INTEGER,
            subdomain TEXT
        )",
        [],
    )?;
    // Create indexes for common queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_check_ins_x_id ON writer(x_id)",
        [],
    )?;
    Ok(())
}

pub fn init_bytes_db(conn: &DbConnection) -> Result<(), WriterError> {
    // Create the check_ins table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bytes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            bytes BLOB NOT NULL,
            x_id TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_bytes_x_id ON bytes(x_id)",
        [],
    )?;

    Ok(())
}

pub fn save_packet(
    db: &DbConnection,
    bytes_db: &DbConnection,
    checkin: CheckIn,
) -> Result<i64, WriterError> {
    bytes_db.execute(
        "INSERT INTO bytes (bytes, x_id)
         VALUES (?1, ?2)",
        params![checkin.bytes, checkin.x_id],
    )?;

    db.execute(
        "INSERT INTO writer (x_id, checkin, forwarded, x_forwarded_for, x_real_ip, client_addr)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            checkin.x_id,
            checkin.checkin as u64,
            checkin.ip.forwarded,
            checkin.ip.x_forwarded_for,
            checkin.ip.x_real_ip,
            checkin.ip.client_addr,
        ],
    )?;
    let last_id = db.last_insert_rowid();
    Ok(last_id)
}

pub fn save_parsed_checkin(
    db: &DbConnection,
    id: i64,
    checkin: ParsedCheckIn,
) -> Result<(), WriterError> {
    db.execute(
        "UPDATE writer SET
            url = ?1,
            query = ?2,
            ua = ?3,
            method = ?4,
            referrer = ?5,
            host = ?6,
            os = ?7,
            browser = ?8,
            is_bot = ?9,
            lead = ?10,
            campaign = ?11,
            anon_name = ?12,
            subdomain = ?13,
            client_ip = ?14,
            client_port = ?15

        WHERE id = ?16",
        params![
            checkin.url,
            checkin.query,
            checkin.ua,
            checkin.method,
            checkin.referrer,
            checkin.host,
            checkin.os,
            checkin.browser,
            checkin.is_bot,
            checkin.lead,
            checkin.campaign,
            checkin.anon_name,
            checkin.subdomain,
            checkin.client_ip,
            checkin.client_port,
            id,
        ],
    )?;
    Ok(())
}

pub fn save_checkout(db: &DbConnection, checkout: CheckOut) -> Result<(), WriterError> {
    db.execute(
        "UPDATE writer SET
            checkout = ?1,
            error = ?2,
            status = ?3,
            timer = ?1 - checkin
        WHERE x_id = ?4",
        params![
            checkout.checkout as u64,
            checkout.error,
            checkout.status,
            checkout.x_id,
        ],
    )?;
    Ok(())
}

use r2d2_sqlite::rusqlite::OptionalExtension;

#[derive(Debug, Clone)]
pub struct EntryResult {
    pub status: Option<i16>,
    pub error: Option<String>,
    pub anon_name: Option<String>,
    pub url: Option<String>,
    pub timer: Option<i64>,
    pub host: Option<String>,
}

pub fn get_one_entry(db: &DbConnection, x_id: &str) -> Result<Option<EntryResult>, WriterError> {
    let entry = db
        .query_row(
            "SELECT status, error, anon_name, timer, url, host
             FROM writer
             WHERE x_id = ?1",
            params![x_id],
            |row| {
                Ok(EntryResult {
                    status: row.get(0)?,
                    error: row.get(1)?,
                    anon_name: row.get(2)?,
                    timer: row.get(3)?,
                    url: row.get(4)?,
                    host: row.get(5)?,
                })
            },
        )
        .optional()?;

    Ok(entry)
}
