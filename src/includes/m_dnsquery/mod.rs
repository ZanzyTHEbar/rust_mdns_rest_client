#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use futures_util::{ pin_mut, stream::StreamExt };
use mdns::{ Error, Record, RecordKind };
use std::{ net::IpAddr, time::Duration };
use log::{ debug, info, warn, error };

/// The hostname of the devices we are searching for.
/// Every Chromecast will respond to the service name in this example.
const SERVICE_NAME: &'static str = "_waterchamber._tcp.local";

#[async_std::main]
pub async fn main() -> Result<(), Error> {
    // Iterate through responses from each Cast device, asking for new devices every 15s
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        let addr = response.records().filter_map(self::to_ip_addr).next();
        info!("Found device: {:?} at {:?}", response.hostname(), addr);
        for record in response.records() {
            info!("Record: {:?}", record);
        }
        if let Some(addr) = addr {
            info!("found openiris device at {}", addr);
        } else {
            warn!("openiris device does not advertise address");
        }
    }

    Ok(())
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}