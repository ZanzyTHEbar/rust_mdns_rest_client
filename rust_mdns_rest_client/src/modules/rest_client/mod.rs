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
use std::sync::{Arc, Mutex};

//use lazy_static::lazy_static;

/* lazy_static! {
    static ref ;
} */

/// A struct to hold the REST client response
/// - `response`: a hashmap of the response
#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "timestamp")]
    timestamp: String,

    #[serde(rename = "max_temp")]
    max_temp: i64,

    #[serde(rename = "num_temp_sensors")]
    num_temp_sensors: i64,

    #[serde(rename = "water_level_liters")]
    water_level_liters: f64,

    #[serde(rename = "water_level_percentage")]
    water_level_percentage: f64,

    #[serde(rename = "humidity_dht")]
    humidity_dht: i64,

    #[serde(rename = "humidity_temp_dht")]
    humidity_temp_dht: f64,

    #[serde(rename = "temp_sensors")]
    temp_sensors: Vec<f64>,
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
}

/// A function to create a new RESTClient instance
/// ## Arguments
/// - `base_url` The base url of the api to query
impl RESTClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
    }
}

pub async fn request(rest_client: &RESTClient) -> Result<(), Box<dyn std::error::Error>> {
    info!("Making REST request");
    let response = rest_client
        .http_client
        .get(&rest_client.base_url)
        .send()
        .await?
        .json::<Response>()
        .await?;
    info!("Response: {:?}", response);
    Ok(())
}

/// A function to run a mDNS query and create a new RESTClient instance for each device found
/// ## Arguments
/// - `service_type` The service type to query for
/// - `scan_time` The number of seconds to query for
pub async fn run_mdns_query(
    service_type: String,
    scan_time: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting MDNS query to find devices");
    let base_url = Arc::new(Mutex::new(HashMap::new()));
    let thread_arc = base_url.clone();
    let mut mdns: m_dnsquery::Mdns = m_dnsquery::Mdns {
        base_url: thread_arc,
        names: Vec::new(),
    };
    let ref_mdns = &mut mdns;

    info!("Thread 1 acquired lock");
    m_dnsquery::run_query(ref_mdns, service_type, scan_time)
        .await
        .expect("Error in mDNS query");
    info!("MDNS query complete");
    info!(
        "MDNS query results: {:#?}",
        m_dnsquery::get_urls(&*ref_mdns)
    ); // get's an array of the base urls found
    m_dnsquery::generate_json(&*ref_mdns).await?; // generates a json file with the base urls found
    Ok(())
}

/// A function to run a REST Client and create a new RESTClient instance for each device found
/// ## Arguments
/// - `service_type` The service type to query for
/// - `scan_time` The number of seconds to query for
pub async fn run_rest_client(endpoint: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting REST client");
    // read the json config file
    let data = std::fs::read_to_string("config/config.json").expect("Unable to read config file");
    // parse the json config file
    let config: serde_json::Value =
        serde_json::from_str(&data).expect("Unable to parse config file");
    debug!("Urls: {:?}", config);
    // create iterator for loop
    for (i, item) in config.as_object().iter().enumerate() {
        // create a new RESTClient instance for each url
        let full_url = format!(
            "{}{}",
            item["urls"][i].as_str().expect("Unable to parse url"),
            endpoint
        );
        //info!("Full url: {}", full_url);
        let rest_client = RESTClient::new(full_url);
        //request(&rest_client).await.expect("Error in REST request");
        // start a new thread for each RESTClient instance
        // pass the response to the main thread
        let thread = tokio::spawn(async move {
            loop {
                request(&rest_client).await.expect("Error in REST request");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
        thread.await.expect("Error in thread");
    }
    Ok(())
}
