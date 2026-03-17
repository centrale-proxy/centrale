use crate::one_connection::one_connection;
use config::CentraleConfig;
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;

const SERVER: Token = Token(0);

pub fn listen_to_port() -> Result<(), Box<dyn Error>> {
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
                    let (connection, address) = server.accept()?;
                    one_connection(&connection, address);
                }
                _ => unreachable!(),
            }
        }
    }
}
