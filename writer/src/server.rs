use crate::one_connection::one_connection;
use crate::poll::get_server_poll;
use config::CentraleConfig;
use dir_and_db_pool::db::DbBool;
use mio::{Events, Token};
use std::error::Error;

const SERVER: Token = Token(0);

pub fn start_server(db: DbBool) -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let (mut poll, server) = get_server_poll(SERVER)?;
    let mut events = Events::with_capacity(CentraleConfig::WRITER_EVENTS_CAPACITY);

    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;
        // Process each event.
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let (connection, address) = server.accept()?;
                    one_connection(&connection, address, &db);
                }
                _ => unreachable!(),
            }
        }
    }
}
