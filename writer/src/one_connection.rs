use crate::one_message::one_message;
use mio::net::TcpStream;
use std::net::SocketAddr;

pub fn one_connection(connection: &TcpStream, address: SocketAddr) {
    println!("Got a connection from: {}", address);
    loop {
        match one_message(&connection) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("errr {:?}", err);
                break;
            }
        }
    }
}
