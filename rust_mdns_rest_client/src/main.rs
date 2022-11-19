#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod modules;
use log::info;
use std::collections::hash_map::HashMap;

fn main() {
    env_logger::init();
    info!("Starting up");
    // call the REST client
    modules::rest_client::main().expect("REST client failed");
}