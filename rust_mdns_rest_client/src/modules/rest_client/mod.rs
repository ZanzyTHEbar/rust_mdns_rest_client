#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// 1. Grab the mDNS address of the device(s) you want to query
// 2. Create a new RESTClient instance for each device
// 3. Start a new thread for each RESTClient instance
// 4. Each thread will poll the device for new data
// 5. Each thread will send the new data to the main thread
// 6. The main thread will update the UI with the new data

use reqwest::Client;
use log::{ debug, info, warn, error };
use std::collections::hash_map::HashMap;
use crate::modules::m_dnsquery;

pub struct RESTClient {
    http_client: Client,
    base_url: String,
}

impl RESTClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mdns: m_dnsquery::Mdns = m_dnsquery::Mdns {
        base_url: HashMap::new(),
    };

    let ref_mdns = &mut mdns;

    info!("Starting MDNS query");
    m_dnsquery::run_query(ref_mdns, String::from("_waterchamber._tcp"), 10);
    info!("MDNS query complete");
    info!("MDNS query results: {:#?}", m_dnsquery::get_urls(&mdns));
    info!("Starting up REST clients");
    debug!("Instatiating REST client for device 1");
    //let mut rest_client = RESTClient::new();

    debug!("Instatiating REST client for device 2");

    Ok(())
}