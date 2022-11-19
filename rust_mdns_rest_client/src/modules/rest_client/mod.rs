#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 1. Grab the mDNS address of the device(s) you want to query
// 2. Create a new RESTClient instance for each device
// 3. Start a new thread for each RESTClient instance
// 4. Each thread will poll the device for new data
// 5. Each thread will send the new data to the main thread
// 6. The main thread will update the UI with the new data

use crate::modules::m_dnsquery;
use log::{debug, error, info, warn};
use reqwest::Client;
use serde::Deserialize;
use std::collections::hash_map::HashMap;
//use lazy_static::lazy_static;

/* lazy_static! {
    static ref ;
} */

/// A struct to hold the REST client response
/// - `response`: a hashmap of the response
#[derive(Deserialize, Debug)]
pub struct Response {
    pub response: String,
}

/// A struct to hold the REST client
/// ## Fields
/// - `client`: a reqwest client
/// - `base_url`: the base url of the api to query
/// - `name`: the name of the url to query
/// - `data`: a hashmap of the data returned from the api
pub struct RESTClient {
    http_client: Client,
    base_url: String,
    pub name: String,
    pub data: HashMap<String, String>,
}

/// A function to create a new RESTClient instance
/// ## Arguments
/// - `base_url` The base url of the api to query
impl RESTClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            name: String::new(),
            data: HashMap::new(),
        }
    }
}

#[tokio::main]
pub async fn request() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting REST request");

    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mdns: m_dnsquery::Mdns = m_dnsquery::Mdns {
        base_url: HashMap::new(),
        name: Vec::new(),
    };

    let ref_mdns = &mut mdns;

    info!("Starting MDNS query to find devices");
    m_dnsquery::run_query(ref_mdns, String::from("_waterchamber._tcp"), 10)
        .expect("Failed to run MDNS query");
    info!("MDNS query complete");
    info!("MDNS query results: {:#?}", m_dnsquery::get_urls(ref_mdns)); // get's an array of the base urls found
    info!("Starting up REST clients");
    debug!("Instatiating REST client for device 1");
    let urls_map = m_dnsquery::get_url_map(ref_mdns);
    let rest_client = RESTClient::new(urls_map.remove("waterchamber").unwrap());

    debug!("Instatiating REST client for device 2");

    Ok(())
}
