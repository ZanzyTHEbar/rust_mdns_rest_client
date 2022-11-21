#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod modules;
use log::info;
use modules::rest_client;
use std::collections::hash_map::HashMap;

#[tokio::main]
async fn main() {
    // create variables
    let endpoint: String = String::from("/api/v1/builtin/command/json?type=data");
    let service_type: String = String::from("_waterchamber._tcp");
    let scan_time: u64 = 10;

    env_logger::init();
    info!("Starting up");
    // call the REST client
    rest_client::run_mdns_query(service_type, scan_time)
        .await
        .expect("Error in MDNS client");
    // call the REST client
    rest_client::run_rest_client(endpoint)
        .await
        .expect("Error in REST client");
}
