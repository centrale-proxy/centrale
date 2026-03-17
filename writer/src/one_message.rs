use crate::convert::vector_to_string;
use crate::error::WriterError;
use crate::payload::WriterPayload;
use mio::net::TcpStream;
use std::io::Read;

pub fn one_message(mut connection: &TcpStream) -> Result<(), WriterError> {
    let mut buffer = [0; 1024];
    //let m = connection.read(&mut buffer);
    match connection.read(&mut buffer) {
        Ok(0) => {
            //println!("Downstream closed");
            // SERVER SEND CLOSE
            return Err(WriterError::DownstreamClosed);
        }
        Ok(n) => {
            let str = vector_to_string(&buffer[..n])?;
            //println!("str {:?}", str);
            //let payload: Result<WriterPayload, serde_json::Error>
            println!("str: {:?}", &str);
            let payload: WriterPayload = serde_json::from_str(&str)?;
            println!("payload: {:?}", &payload);
            match payload {
                WriterPayload::CheckIn => {
                    println!("checkin: {:?}", payload);
                }
                WriterPayload::CheckOut => {
                    println!("checkout: {:?}", payload);
                }
            }
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
