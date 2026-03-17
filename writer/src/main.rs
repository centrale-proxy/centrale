mod convert;
mod error;
mod listen;
mod one_message;
mod payload;

use crate::listen::listen_to_port;

// GET DB
// START SERVER
// OPEN CHANNEL
// RECEIVE MESSAGE

fn main() {
    let _li = listen_to_port();
}
