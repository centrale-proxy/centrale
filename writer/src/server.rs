use crate::poll::get_server_poll;
use crate::save_to_db::save_to_db;
use common::payload::WriterPayload;
use config::CentraleConfig;
use dir_and_db_pool::db::DbPool;
use mio::{Events, Token};
use std::error::Error;
use std::io::ErrorKind;
use std::str::from_utf8;

const SERVER: Token = Token(0);

pub fn start_server(pool: DbPool) -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let (mut poll, server) = get_server_poll(SERVER)?;
    let mut events = Events::with_capacity(CentraleConfig::WRITER_EVENTS_CAPACITY);
    let mut buf = [0u8; 5120];

    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let db = pool.get().expect("Couldn't get db connection from pool");
                    match server.recv_from(&mut buf) {
                        Ok((len, _src)) => {
                            let msg = from_utf8(&buf[..len]).unwrap_or("<invalid utf8>");
                            let payload: WriterPayload = serde_json::from_str(&msg)?;
                            save_to_db(payload, &db);
                        }
                        Err(e) if e.kind() == ErrorKind::WouldBlock => {
                            break; // No more packets ready — back to poll
                        }
                        Err(e) => {
                            eprintln!("writer loop err: {}:?", e);
                            break;
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}
