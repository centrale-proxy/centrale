mod convert;
mod error;
mod listen;
mod one_connection;
mod one_message;
mod payload;

use crate::listen::listen_to_port;

// GET DB
// START SERVER
// OPEN CHANNEL
// RECEIVE MESSAGE

fn main() {
    match listen_to_port() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Writer Err: {}", err);
        }
    }
}
