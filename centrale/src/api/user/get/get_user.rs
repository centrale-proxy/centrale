use crate::{api::user::get::get::GetUser, error::CentraleError};
use dir_and_db_pool::db::DbConnection;
use r2d2_sqlite::rusqlite::{OptionalExtension, params};

/// Get user from db by id
pub fn get_user_from_db(db: &DbConnection, id: i64) -> Result<Option<GetUser>, CentraleError> {
    let user = db
        .query_row(
            "SELECT id, username, name, first_name, last_name, personal_code, email
             FROM user WHERE id = ?1",
            params![id],
            |row| {
                Ok(GetUser {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    name: row.get(2)?,
                    first_name: row.get(3)?,
                    last_name: row.get(4)?,
                    personal_code: row.get(5)?,
                    email: row.get(6)?,
                })
            },
        )
        .optional()?;
    Ok(user)
}
