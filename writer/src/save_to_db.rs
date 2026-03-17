use crate::payload::WriterPayload;

pub fn save_to_db(payload: WriterPayload) {
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
