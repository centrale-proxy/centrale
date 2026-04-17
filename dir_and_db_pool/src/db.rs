pub mod app_folder;
pub mod create_home_dir;
pub mod db_file;
pub mod encrypted;
pub mod get_db;
pub mod get_encrypted_connection;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type DbPool = Pool<SqliteConnectionManager>;
pub type DbConnection = PooledConnection<SqliteConnectionManager>;
