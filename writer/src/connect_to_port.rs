use config::CentraleConfig;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;
use std::io::Write;

pub fn get_client_poll() -> Result<(Poll, TcpStream), Box<dyn Error>> {
    const CLIENT: Token = Token(1);
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

pub fn connect_to_port() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let (mut poll, mut client) = get_client_poll()?;
    let mut events = Events::with_capacity(128);

    //loop {
    // Poll Mio for events, blocking until we get an event.
    poll.poll(&mut events, None)?;

    for event in events.iter() {
        if event.is_writable() {
            /*
            let co = CheckOut {};
            let pl = WriterPayload::CheckOut(co);
            let sss = serde_json::to_string(&pl).unwrap();
            let uu = string_to_vector(&sss);
            */
            let message = b"{\"CheckOut\":{}}";
            client.write_all(&message.to_vec())?;
            client.flush()?;
        }
        //handle_one_event(event, &client)?;
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
        println!("connect to writer: {:?}", a);
    }
}
