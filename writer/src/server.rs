use crate::handle_payload::handle_payload;
use crate::packet::init_writer_db;
use crate::poll::get_server_poll;
use common::payload::WriterPayload;
use config::CentraleConfig;
use dir_and_db_pool::db::DbPool;
use mio::{Events, Token};
use std::error::Error;
use std::io::ErrorKind;

const SERVER: Token = Token(0);

pub fn start_server(pool: DbPool) -> Result<(), Box<dyn Error>> {
    // INIT WRITER DB
    let db = pool.get()?;
    init_writer_db(&db)?;
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
                            let packet = &buf[..len];
                            if let Ok(payload) = serde_json::from_slice::<WriterPayload>(packet) {
                                match handle_payload(payload, &db) {
                                    Err(err) => {
                                        eprint!("payload handle error {}", err);
                                    }
                                    _ => {}
                                }
                            } else {
                                // unable to parse
                                break;
                            }
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
