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
        WriterPayload::CheckIn(checkin) => {
            // SAVE INITIAL DATA
            let id = save_packet(db, checkin.clone())?;
            // PARSE
            let parsed = ParsedCheckIn::parse_checkin(&checkin);
            // SAVE
            save_parsed_checkin(db, id, parsed.clone())?;
            println!(
                "> {} {} {} {}",
                parsed.method.unwrap_or("".to_string()),
                parsed.url.unwrap_or("".to_string()),
                checkin.ip.unwrap_or("".to_string()),
                checkin.checkin,
            );
        }
        WriterPayload::CheckOut(checkout) => {
            save_checkout(db, checkout.clone())?;
            println!(
                "< {} {} {}",
                checkout.status.unwrap_or(0),
                checkout.error.unwrap_or("".to_string()),
                checkout.checkout,
            );
        }
    }
    Ok(())
}
