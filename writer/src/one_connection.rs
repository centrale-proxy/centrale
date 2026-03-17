use crate::one_message::one_message;
use dir_and_db_pool::db::DbBool;
use mio::net::TcpStream;
use std::net::SocketAddr;

pub fn one_connection(connection: &TcpStream, address: SocketAddr, pool: &DbBool) {
    println!("Got a connection from: {}", address);
    let db = pool.get().expect("Couldn't get db connection from pool");

    loop {
        match one_message(&connection, &db) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("errr {:?}", err);
                break;
            }
        }
    }
}
