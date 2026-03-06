use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub mod app_folder;
pub mod create_home_dir;
pub mod db_file;
pub mod get_db;

pub type DbBool = Pool<SqliteConnectionManager>;
pub type DbConnection = PooledConnection<SqliteConnectionManager>;
