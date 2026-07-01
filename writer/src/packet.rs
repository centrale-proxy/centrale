use crate::error::WriterError;
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::params;

pub fn init_writer_db(conn: &DbConnection) -> Result<(), WriterError> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS writer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            data BLOB,
            text TEXT,
            error TEXT
        );
        ",
    )?;
    Ok(())
}

pub fn post_packet(db: &DbConnection, data: &[u8]) -> Result<i64, WriterError> {
    //
    db.execute("INSERT INTO writer (data) VALUES (?1)", params![data])?;
    let last_id = db.last_insert_rowid();
    Ok(last_id)
}

pub fn update_packet(db: &DbConnection, id: i64, text: &str) -> Result<(), WriterError> {
    db.execute(
        "UPDATE writer SET text = ?1 WHERE id = ?2",
        params![text, id],
    )?;

    Ok(())
}
