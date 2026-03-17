use crate::convert::vector_to_string;
use crate::error::WriterError;
use crate::payload::WriterPayload;
use crate::save_to_db::save_to_db;
use dir_and_db_pool::db::DbConnection;
use mio::net::TcpStream;
use std::io::Read;

pub fn one_message(mut connection: &TcpStream, db: &DbConnection) -> Result<(), WriterError> {
    let mut buffer = [0; 1024];
    //let m = connection.read(&mut buffer);
    match connection.read(&mut buffer) {
        Ok(0) => {
            // println!("Downstream closed");
            // SERVER SEND CLOSE
            return Err(WriterError::ClientClosed);
        }
        Ok(n) => {
            let str = vector_to_string(&buffer[..n])?;
            println!("str: {:?}", &str);
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
