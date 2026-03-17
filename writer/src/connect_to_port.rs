use config::CentraleConfig;
use mio::event::Event;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;
use std::io::Write;

use crate::convert::string_to_vector;
use crate::error::WriterError;
use crate::payload::{CheckOut, WriterPayload};

const CLIENT: Token = Token(1);

pub fn get_client_poll() -> Result<(Poll, TcpStream), Box<dyn Error>> {
    // Create a poll instance.
    let poll = Poll::new()?;
    // Create storage for events.
    // Setup the server socket.
    let addr = CentraleConfig::WRITER_SERVER_ADDRESS.parse()?;

    let mut client = TcpStream::connect(addr)?;

    // Start listening for incoming connections.
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    Ok((poll, client))
}

pub fn handle_one_event(event: &Event, mut client: &TcpStream) -> Result<(), WriterError> {
    match event.token() {
        CLIENT => {
            if event.is_writable() {
                // println!("is writable");
                // let message = b"Hello, server!";
                //let message = b"WriterPayload(CheckOut(CheckOut{}))";
                let _message = b"{\"CheckOut\":{}}";

                let co = CheckOut {};
                let pl = WriterPayload::CheckOut(co);
                let sss = serde_json::to_string(&pl).unwrap();
                let uu = string_to_vector(&sss);
                client.write_all(&uu)?;
                client.flush()?;
                // We can (likely) write to the socket without blocking.
                return Ok(());
            }

            Ok(())

            // if event.is_readable() {
            // We can (likely) read from the socket without blocking.
            // }

            // return Ok(());
        }

        _ => unreachable!(),
    }
}

pub fn connect_to_port() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let (mut poll, client) = get_client_poll()?;
    let mut events = Events::with_capacity(128);

    //loop {
    // Poll Mio for events, blocking until we get an event.
    poll.poll(&mut events, None)?;

    for event in events.iter() {
        handle_one_event(event, &client)?;
    }

    Ok(())
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_writer() {
        let a = connect_to_port().unwrap();
        println!("{:?}", a);
    }
}
