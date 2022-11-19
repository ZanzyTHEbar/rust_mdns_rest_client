#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod includes;

use log::info;
use includes::m_dnsquery;

fn main() {
    env_logger::init();

    let mut mdns:m_dnsquery::MDNS = m_dnsquery::MDNS {
        base_url: std::collections::HashMap::new(),
    };

    let ref_mdns = &mut mdns;

    info!("Starting MDNS query");
    m_dnsquery::run_query(ref_mdns, String::from("_waterchamber._tcp"), 10);
    info!("MDNS query complete");
    info!("MDNS query results: {:#?}", m_dnsquery::get_urls(&mdns));
}