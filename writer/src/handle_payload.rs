use std::collections::HashMap;

use crate::{
    db::{get_one_entry, save_checkout, save_packet, save_parsed_checkin, update_counter},
    error::WriterError,
    parse_checkin::ParsedCheckIn,
};
use common::{names::RandomName, payload::WriterPayload};
use dir_and_db_pool::db::DbConnection;

pub fn handle_payload(
    payload: WriterPayload,
    db: &DbConnection,
    bytes_db: &DbConnection,
    names: &mut HashMap<String, String>,
    feed_tx: &tokio::sync::broadcast::Sender<String>,
) -> Result<(), WriterError> {
    // println!("payload: {:?}", &payload);
    match payload {
        WriterPayload::CheckIn(checkin) => {
            // SAVE INITIAL DATA
            let id = save_packet(db, bytes_db, checkin.clone())?;
            // PARSE
            let ip = checkin.ip.for_logging();
            let ip_only = ip.split(':').next().unwrap_or(&ip).to_string();
            let port_only = ip
                .rsplit(':')
                .next()
                .unwrap_or(&"0".to_string())
                .parse::<u16>()
                .unwrap_or(0);

            let parsed = ParsedCheckIn::parse_checkin(&checkin, names, &ip_only, port_only);
            // SAVE
            save_parsed_checkin(db, id, parsed.clone())?;

            if let Ok(event) = serde_json::to_string(&parsed) {
                // Sending without subscribers is expected and should not fail the check-in.
                let _ = feed_tx.send(event);
            }

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
            let entry = get_one_entry(db, &checkout.x_id)?;
            match entry {
                Some(entry) => {
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
                None => {
                    eprintln!("entry not found {:?}", checkout);
                }
            }
        }
        WriterPayload::CentralePing(ping) => {
            let ip = ping.ip.to_owned();
            let anon_name = names
                .entry(ip.clone())
                .or_insert_with(|| RandomName::new().name)
                .clone();

            // ADD COUNTER VALUE TO WRITER ENTRY
            update_counter(db, &ip, &ping.url, ping.counter)?;

            println!(
                "  {} {}{} {} {}",
                ping.counter,
                ping.host.unwrap_or("".to_string()),
                ping.url,
                anon_name,
                ping.ip,
            );
        }
    }
    Ok(())
}
