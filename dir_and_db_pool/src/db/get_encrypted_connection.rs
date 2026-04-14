use crate::{
    db::{DbBool, DbConnection},
    error::DirsqlError,
};

pub fn get_encrypted_connection(pool: &DbBool, pass: &str) -> Result<DbConnection, DirsqlError> {
    let db = pool.get().unwrap();
    let query = format!("PRAGMA key = '{}';", pass.replace("'", "''"));
    db.execute_batch(&query)?;

    Ok(db)
}
