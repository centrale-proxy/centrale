use std::collections::HashMap;

use crate::{
    db::{
        get_full_entry, get_one_entry, save_checkout, save_packet, save_parsed_checkin,
        update_counter,
    },
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
            let full_entry = get_full_entry(db, id)?;
            if let Ok(event) = serde_json::to_string(&full_entry) {
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
            let id = save_checkout(db, checkout.clone())?;
            let full_entry = get_full_entry(db, id)?;
            if let Ok(event) = serde_json::to_string(&full_entry) {
                let _ = feed_tx.send(event);
            }

            let entry = get_one_entry(db, &checkout.x_id)?;
            match entry {
                Some(entry) => {
                    let e = entry.clone();
                    println!(
                        "< {} {}{} {} {} {}",
                        e.status.unwrap_or(0),
                        e.host.unwrap_or("".to_string()),
                        e.url.unwrap_or("".to_string()),
                        e.error.unwrap_or("".to_string()),
                        e.anon_name.clone().unwrap_or("".to_string()),
                        e.timer.unwrap_or(0),
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
            let id = update_counter(db, &ip, &ping.url, ping.counter)?;

            let full_entry = get_full_entry(db, id)?;
            if let Ok(event) = serde_json::to_string(&full_entry) {
                let _ = feed_tx.send(event);
            }

            let p = ping.clone();
            println!(
                "  {} {}{} {} {}",
                p.counter,
                p.host.unwrap_or("".to_string()),
                p.url,
                anon_name,
                p.ip,
            );
            //  if let Ok(event) = serde_json::to_string(&ping) {
            //      let _ = feed_tx.send(event);
            //  }
        }
    }
    Ok(())
}
