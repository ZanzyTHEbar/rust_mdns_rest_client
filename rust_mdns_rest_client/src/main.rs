#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod modules;
use log::info;
use std::collections::hash_map::HashMap;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up");
    // call the REST client
    modules::rest_client::run()
        .await
        .expect("Error in REST client");
}
