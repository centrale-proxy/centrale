pub mod app_folder;
pub mod create_home_dir;
pub mod db_file;
pub mod encrypted;
pub mod get_db;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type DbBool = Pool<SqliteConnectionManager>;
pub type DbConnection = PooledConnection<SqliteConnectionManager>;
