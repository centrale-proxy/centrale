use common::payload::WriterPayload;
use dir_and_db_pool::db::DbConnection;

pub fn save_to_db(payload: WriterPayload, db: &DbConnection) {
    // println!("payload: {:?}", &payload);
    match payload {
        WriterPayload::CheckIn(payload) => {
            // SEND TX OR GO STRAIGHT TO DB?
            println!(
                "{} {} {}",
                payload.method.unwrap_or("".to_string()),
                payload.url.unwrap_or("".to_string()),
                payload.checkin
            );
        }
        WriterPayload::CheckOut(payload) => {
            println!(
                "{} {} {}",
                payload.status.unwrap_or(0),
                payload.error.unwrap_or("".to_string()),
                payload.checkout
            );
        }
    }
}
