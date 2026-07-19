use bytes::{Bytes, BytesMut};
use pingora::prelude::*;

const MAX_BODY_SIZE: usize = 3 * 1024; // 3 kb safety cap

pub async fn read_full_body(session: &mut Session) -> pingora::Result<Bytes> {
    let mut body = BytesMut::new();
    loop {
        match session.read_request_body().await? {
            Some(chunk) => {
                body.extend_from_slice(&chunk);
                if body.len() > MAX_BODY_SIZE {
                    return Error::e_explain(ErrorType::ReadError, "request body too large");
                }
            }
            None => break, // end of body
        }
    }
    Ok(body.freeze())
}
