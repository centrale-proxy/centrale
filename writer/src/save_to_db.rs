use crate::parse_checkin::parse_checkin2;
use common::payload::WriterPayload;
use dir_and_db_pool::db::DbConnection;

pub fn save_to_db(payload: WriterPayload, _db: &DbConnection) {
    // println!("payload: {:?}", &payload);
    match payload {
        WriterPayload::CheckIn(payload) => {
            // SEND TX OR GO STRAIGHT TO DB?
            println!(
                "{} {} {} {}",
                payload.method.unwrap_or("".to_string()),
                payload.url.unwrap_or("".to_string()),
                payload.ip.unwrap_or("".to_string()),
                payload.checkin
            );
        }
        WriterPayload::CheckIn2(payload) => {
            // SEND TX OR GO STRAIGHT TO DB?
            //println!("> {}  {:?} {}", payload.x_id, payload.ip, payload.checkin);

            //let text = String::from_utf8_lossy(&payload.bytes);
            let parsed = parse_checkin2(&payload);

            // println!("> {}", text);
            println!("> parsed: {:?}", parsed);
        }
        WriterPayload::CheckOut(payload) => {
            println!(
                "{} {} {}",
                payload.status.unwrap_or(0),
                payload.error.unwrap_or("".to_string()),
                payload.checkout
            );
        }
        WriterPayload::CheckOut2(payload) => {
            println!(
                "< {} {} {} {}",
                payload.status.unwrap_or(0),
                payload.error.unwrap_or("".to_string()),
                payload.checkout,
                payload.x_id
            );
        }
    }
}
