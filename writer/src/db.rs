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
            ip TEXT,
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
            STATUS TEXT,
            bytes TEXT
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

pub fn save_packet(db: &DbConnection, checkin: CheckIn) -> Result<i64, WriterError> {
    //
    db.execute(
        "INSERT INTO writer (bytes, x_id, checkin, ip) VALUES (?1, ?2, ?3, ?4)",
        params![
            checkin.bytes,
            checkin.x_id,
            checkin.checkin as u64,
            checkin.ip
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
            campaign = ?11
        WHERE id = ?12",
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
            status = ?3
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
