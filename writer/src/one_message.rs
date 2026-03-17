use crate::error::WriterError;
use crate::save_to_db::save_to_db;
use common::convert::vector_to_string;
use common::payload::WriterPayload;
use config::CentraleConfig;
use dir_and_db_pool::db::DbConnection;
use mio::net::TcpStream;
use std::io::Read;

pub fn one_message(mut connection: &TcpStream, db: &DbConnection) -> Result<(), WriterError> {
    let mut buffer = [0; CentraleConfig::WRITER_BUFFER_SIZE];
    //let m = connection.read(&mut buffer);
    match connection.read(&mut buffer) {
        Ok(0) => {
            return Err(WriterError::ClientClosed);
        }
        Ok(n) => {
            let str = vector_to_string(&buffer[..n])?;
            let payload: WriterPayload = serde_json::from_str(&str)?;
            save_to_db(payload, db);
            Ok(())
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                // Not an error — just no data available right now
                return Ok(());
            }

            Err(e.into())
        }
    }
}
