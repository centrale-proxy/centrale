use crate::{
    error::WriterError,
    packet::{save_checkout, save_packet, save_parsed_checkin},
    parse_checkin::ParsedCheckIn,
};
use common::payload::WriterPayload;
use dir_and_db_pool::db::DbConnection;

pub fn handle_payload(payload: WriterPayload, db: &DbConnection) -> Result<(), WriterError> {
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
        WriterPayload::CheckIn2(checkin) => {
            // SAVE INITIAL DATA
            let id = save_packet(db, checkin.clone())?;
            // PARSE
            let parsed = ParsedCheckIn::parse_checkin(&checkin);
            // SAVE
            save_parsed_checkin(db, id, parsed.clone())?;
            println!("> {:?}", parsed);
        }
        WriterPayload::CheckOut(payload) => {
            println!(
                "{} {} {}",
                payload.status.unwrap_or(0),
                payload.error.unwrap_or("".to_string()),
                payload.checkout
            );
        }
        WriterPayload::CheckOut2(checkout) => {
            save_checkout(db, checkout.clone())?;
            println!(
                "< {} {} {} {}",
                checkout.status.unwrap_or(0),
                checkout.error.unwrap_or("".to_string()),
                checkout.checkout,
                checkout.x_id
            );
        }
    }
    Ok(())
}
