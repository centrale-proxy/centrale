use config::CentraleConfig;
use mio::net::UdpSocket;
use mio::{Interest, Poll, Token};
use std::error::Error;

pub fn get_server_poll(server_token: Token) -> Result<(Poll, UdpSocket), Box<dyn Error>> {
    let poll = Poll::new()?;
    // Setup the server socket.
    let addr = CentraleConfig::WRITER_SERVER_ADDRESS.parse()?;
    let mut server = UdpSocket::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, server_token, Interest::READABLE)?;

    Ok((poll, server))
}
