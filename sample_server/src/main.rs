pub mod auth;
pub mod db;
pub mod error;
pub mod hello;
pub mod pool;
pub mod register;
pub mod routes;
pub mod server;

use crate::server::start_server;

fn main() {
    start_server().unwrap();
}
