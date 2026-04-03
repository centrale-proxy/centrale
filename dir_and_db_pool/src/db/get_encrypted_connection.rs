use crate::{
    db::{DbBool, DbConnection},
    error::DirsqlError,
};

pub fn get_encrypted_connection(pool: &DbBool, pass: &str) -> Result<DbConnection, DirsqlError> {
    let db = pool.get().unwrap();
    db.execute_batch(&format!("PRAGMA key = '{}';", pass))?;

    Ok(db)
}
