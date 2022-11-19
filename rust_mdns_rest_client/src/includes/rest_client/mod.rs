#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 1. Grab the mDNS address of the camera(s)
// 2. Create a new RESTClient instance for each camera
// 3. Start a new thread for each RESTClient instance
// 4. Each thread will poll the camera for new data
// 5. Each thread will send the new data to the main thread
// 6. The main thread will update the UI with the new data

use reqwest::Client;
use log::{debug, info, warn, error};

pub struct RESTClient {
  http_client: Client,
  base_url: String,
}

impl RESTClient {
  pub fn new(base_url: String) -> Self {
    Self {
      http_client: Client::new(),
      base_url
    }
  }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  info!("Starting up REST clients");
  debug!("Instatiating REST client for camera 1");
  //let mut rest_client = RESTClient::new();

  debug!("Instatiating REST client for camera 2");

  Ok(())
}