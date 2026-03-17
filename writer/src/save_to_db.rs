use common::payload::WriterPayload;
use dir_and_db_pool::db::DbConnection;

pub fn save_to_db(payload: WriterPayload, db: &DbConnection) {
    println!("payload: {:?}", &payload);
    match payload {
        WriterPayload::CheckIn(payload) => {
            // SEND TX OR GO STRAIGHT TO DB?
            println!("checkin: {:?}", payload);
        }
        WriterPayload::CheckOut(payload) => {
            println!("checkout: {:?}", payload);
        }
    }
}
