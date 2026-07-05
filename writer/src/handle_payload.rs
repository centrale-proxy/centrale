use std::collections::HashMap;

use crate::{
    db::{get_one_entry, save_checkout, save_packet, save_parsed_checkin},
    error::WriterError,
    parse_checkin::ParsedCheckIn,
};
use common::payload::WriterPayload;
use dir_and_db_pool::db::DbConnection;

pub fn handle_payload(
    payload: WriterPayload,
    db: &DbConnection,
    bytes_db: &DbConnection,
    names: &mut HashMap<String, String>,
) -> Result<(), WriterError> {
    // println!("payload: {:?}", &payload);
    match payload {
        WriterPayload::CheckIn(checkin) => {
            // SAVE INITIAL DATA
            let id = save_packet(db, bytes_db, checkin.clone())?;
            // PARSE
            let parsed = ParsedCheckIn::parse_checkin(&checkin, names);
            // SAVE
            save_parsed_checkin(db, id, parsed.clone())?;

            println!(
                "> {} {}{}  {}",
                parsed.method.unwrap_or("".to_string()),
                parsed.host.unwrap_or("".to_string()),
                parsed.url.unwrap_or("".to_string()),
                parsed.anon_name,
            );
        }
        WriterPayload::CheckOut(checkout) => {
            save_checkout(db, checkout.clone())?;
            let entry = get_one_entry(db, &checkout.x_id)?.unwrap();

            println!(
                "< {} {}{} {} {} {}",
                entry.status.unwrap_or(0),
                entry.host.unwrap_or("".to_string()),
                entry.url.unwrap_or("".to_string()),
                entry.error.unwrap_or("".to_string()),
                entry.anon_name.unwrap_or("".to_string()),
                entry.timer.unwrap_or(0),
            );
        }
    }
    Ok(())
}
