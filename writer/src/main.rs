mod convert;
mod error;
mod one_message;
mod payload;

use crate::one_message::one_message;
use config::CentraleConfig;
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;

const SERVER: Token = Token(0);

// GET DB
// START SERVER
// OPEN CHANNEL
// RECEIVE MESSAGE

fn main() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);
    // Setup the server socket.
    let addr = CentraleConfig::WRITER_SERVER_ADDRESS.parse()?;
    let mut server = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let connection = server.accept();
                    // LOOP MESSAGES FROM EACH CONNECTION
                    match connection {
                        Ok((connection, address)) => {
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
                        Err(err) => {
                            eprintln!("eeee {}", err);
                            break;
                        }
                    }
                    //drop(connection);
                    //}
                }
                _ => unreachable!(),
            }
        }
    }
}
