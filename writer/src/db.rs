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
            subdomain TEXT,
            counter INTEGER,
            year INTEGER,
            month INTEGER,
            day INTEGER
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_check_ins_x_id ON writer(x_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_writer_ip_url_checkin
         ON writer(client_ip, url, checkin)",
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

    let id = db.query_row(
        "INSERT INTO writer (x_id, checkin, forwarded, x_forwarded_for, x_real_ip, client_addr)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(x_id) DO UPDATE SET
             checkin = excluded.checkin,
             forwarded = excluded.forwarded,
             x_forwarded_for = excluded.x_forwarded_for,
             x_real_ip = excluded.x_real_ip,
             client_addr = excluded.client_addr,
             timer = CASE
                 WHEN writer.checkout IS NULL THEN NULL
                 ELSE writer.checkout - excluded.checkin
             END
         RETURNING id",
        params![
            checkin.x_id,
            checkin.checkin as u64,
            checkin.ip.forwarded,
            checkin.ip.x_forwarded_for,
            checkin.ip.x_real_ip,
            checkin.ip.client_addr,
        ],
        |row| row.get(0),
    )?;
    Ok(id)
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
            method = ?3,
            referrer = ?4,
            host = ?5,
            os = ?6,
            browser = ?7,
            is_bot = ?8,
            lead = ?9,
            campaign = ?10,
            anon_name = ?11,
            subdomain = ?12,
            client_ip = ?13,
            client_port = ?14,
            year = ?15,
            month = ?16,
            day = ?17

        WHERE id = ?18",
        params![
            checkin.url,
            checkin.query,
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
            checkin.year,
            checkin.month,
            checkin.day,
            id,
        ],
    )?;
    Ok(())
}

pub fn save_checkout(db: &DbConnection, checkout: CheckOut) -> Result<i64, WriterError> {
    let id = db.query_row(
        "INSERT INTO writer (x_id, checkin, checkout, error, status, timer)
         VALUES (?1, ?2, ?2, ?3, ?4, 0)
         ON CONFLICT(x_id) DO UPDATE SET
             checkout = excluded.checkout,
             error = excluded.error,
             status = excluded.status,
             timer = excluded.checkout - writer.checkin
         RETURNING id",
        params![
            checkout.x_id,
            checkout.checkout as u64,
            checkout.error,
            checkout.status,
        ],
        |row| row.get(0),
    )?;
    Ok(id)
}

use r2d2_sqlite::rusqlite::OptionalExtension;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryResult {
    pub id: i64,
    pub status: Option<i16>,
    pub error: Option<String>,
    pub anon_name: Option<String>,
    pub url: Option<String>,
    pub timer: Option<i64>,
    pub host: Option<String>,
}

pub fn get_bytes(db: &DbConnection, x_id: &str) -> Result<Option<Vec<u8>>, WriterError> {
    let bytes = db
        .query_row(
            "SELECT bytes
             FROM bytes
             WHERE x_id = ?1
             ORDER BY id DESC
             LIMIT 1",
            params![x_id],
            |row| row.get(0),
        )
        .optional()?;

    Ok(bytes)
}

pub fn get_one_entry(db: &DbConnection, x_id: &str) -> Result<Option<EntryResult>, WriterError> {
    let entry = db
        .query_row(
            "SELECT status, error, anon_name, timer, url, host, id
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
                    id: row.get(6)?,
                })
            },
        )
        .optional()?;

    Ok(entry)
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn update_counter(
    db: &DbConnection,
    client_ip: &str,
    url: &str,
    counter: u16,
) -> Result<i64, WriterError> {
    if let Some(id) = find_entry_id(db, client_ip, url)? {
        db.execute(
            "UPDATE writer
             SET counter = ?1
             WHERE id = ?2",
            params![counter, id],
        )?;
        Ok(id)
    } else {
        println!("entry not found for ping {} {} {}", client_ip, url, counter);
        Err(WriterError::StringError("NotFound".to_string()))
    }
}

pub fn find_entry_id(
    db: &DbConnection,
    client_ip: &str,
    url: &str,
) -> Result<Option<i64>, WriterError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let cutoff = now - 10 * 60; // 10 minutes ago

    let entry = db
        .query_row(
            "SELECT id
             FROM writer
             WHERE client_ip = ?1
               AND url = ?2
               AND checkin >= ?3
             ORDER BY checkin DESC
             LIMIT 1",
            params![client_ip, url, cutoff],
            |row| Ok(row.get(0)?),
        )
        .optional()?;

    Ok(entry)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullEntryResult {
    pub id: i64,
    pub x_id: String,
    pub forwarded: Option<String>,
    pub x_forwarded_for: Option<String>,
    pub x_real_ip: Option<String>,
    pub client_addr: Option<String>,
    pub client_ip: Option<String>,
    pub client_port: Option<i64>,
    pub url: Option<String>,
    pub query: Option<String>,
    pub method: Option<String>,
    pub referrer: Option<String>,
    pub host: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub is_bot: bool,
    pub lead: Option<String>,
    pub campaign: Option<String>,
    pub checkin: i64,
    pub checkout: Option<i64>,
    pub error: Option<String>,
    pub status: Option<i16>,
    pub anon_name: Option<String>,
    pub timer: Option<i64>,
    pub subdomain: Option<String>,
    pub counter: Option<i64>,
}

pub fn get_full_entry(db: &DbConnection, id: i64) -> Result<Option<FullEntryResult>, WriterError> {
    let entry = db
        .query_row(
            "SELECT id, x_id, forwarded, x_forwarded_for, x_real_ip, client_addr,
                    client_ip, client_port, url, query, method, referrer, host,
                    os, browser, is_bot, lead, campaign, checkin, checkout, error,
                    status, anon_name, timer, subdomain, counter
             FROM writer
             WHERE id = ?1",
            params![id],
            |row| {
                Ok(FullEntryResult {
                    id: row.get(0)?,
                    x_id: row.get(1)?,
                    forwarded: row.get(2)?,
                    x_forwarded_for: row.get(3)?,
                    x_real_ip: row.get(4)?,
                    client_addr: row.get(5)?,
                    client_ip: row.get(6)?,
                    client_port: row.get(7)?,
                    url: row.get(8)?,
                    query: row.get(9)?,
                    method: row.get(10)?,
                    referrer: row.get(11)?,
                    host: row.get(12)?,
                    os: row.get(13)?,
                    browser: row.get(14)?,
                    is_bot: row.get(15)?,
                    lead: row.get(16)?,
                    campaign: row.get(17)?,
                    checkin: row.get(18)?,
                    checkout: row.get(19)?,
                    error: row.get(20)?,
                    status: row.get(21)?,
                    anon_name: row.get(22)?,
                    timer: row.get(23)?,
                    subdomain: row.get(24)?,
                    counter: row.get(25)?,
                })
            },
        )
        .optional()?;

    Ok(entry)
}

pub fn get_last_entries(
    db: &DbConnection,
    limit: i64,
) -> Result<Vec<FullEntryResult>, WriterError> {
    let mut stmt = db.prepare(
        "SELECT id, x_id, forwarded, x_forwarded_for, x_real_ip, client_addr,
                client_ip, client_port, url, query, method, referrer, host,
                os, browser, is_bot, lead, campaign, checkin, checkout, error,
                status, anon_name, timer, subdomain, counter
         FROM writer
         ORDER BY id DESC
         LIMIT ?1",
    )?;

    let entries = stmt
        .query_map(params![limit], |row| {
            Ok(FullEntryResult {
                id: row.get(0)?,
                x_id: row.get(1)?,
                forwarded: row.get(2)?,
                x_forwarded_for: row.get(3)?,
                x_real_ip: row.get(4)?,
                client_addr: row.get(5)?,
                client_ip: row.get(6)?,
                client_port: row.get(7)?,
                url: row.get(8)?,
                query: row.get(9)?,
                method: row.get(10)?,
                referrer: row.get(11)?,
                host: row.get(12)?,
                os: row.get(13)?,
                browser: row.get(14)?,
                is_bot: row.get(15)?,
                lead: row.get(16)?,
                campaign: row.get(17)?,
                checkin: row.get(18)?,
                checkout: row.get(19)?,
                error: row.get(20)?,
                status: row.get(21)?,
                anon_name: row.get(22)?,
                timer: row.get(23)?,
                subdomain: row.get(24)?,
                counter: row.get(25)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::{
        get_bytes, get_full_entry, init_bytes_db, init_writer_db, save_checkout, save_packet,
    };
    use common::{
        client_ip::ClientIP,
        payload::{CheckIn, CheckOut},
    };
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;

    #[test]
    fn checkin_completes_row_when_checkout_arrives_first() {
        let db_pool = Pool::builder()
            .max_size(1)
            .build(SqliteConnectionManager::memory())
            .unwrap();
        let bytes_pool = Pool::builder()
            .max_size(1)
            .build(SqliteConnectionManager::memory())
            .unwrap();
        let db = db_pool.get().unwrap();
        let bytes_db = bytes_pool.get().unwrap();
        init_writer_db(&db).unwrap();
        init_bytes_db(&bytes_db).unwrap();

        let checkout_id = save_checkout(
            &db,
            CheckOut {
                checkout: 1_500,
                error: None,
                status: Some(204),
                x_id: "out-of-order".to_string(),
            },
        )
        .unwrap();
        let checkin_id = save_packet(
            &db,
            &bytes_db,
            CheckIn {
                checkin: 1_000,
                ip: ClientIP {
                    client_addr: Some("127.0.0.1:1234".to_string()),
                    ..ClientIP::default()
                },
                bytes: b"GET / HTTP/1.1\r\n\r\n".to_vec(),
                host: None,
                x_id: "out-of-order".to_string(),
            },
        )
        .unwrap();

        assert_eq!(checkin_id, checkout_id);
        let entry = get_full_entry(&db, checkin_id).unwrap().unwrap();
        assert_eq!(entry.checkin, 1_000);
        assert_eq!(entry.checkout, Some(1_500));
        assert_eq!(entry.status, Some(204));
        assert_eq!(entry.timer, Some(500));
        assert_eq!(entry.client_addr.as_deref(), Some("127.0.0.1:1234"));
        assert_eq!(
            get_bytes(&bytes_db, "out-of-order").unwrap(),
            Some(b"GET / HTTP/1.1\r\n\r\n".to_vec())
        );
        assert_eq!(get_bytes(&bytes_db, "missing").unwrap(), None);
    }
}
