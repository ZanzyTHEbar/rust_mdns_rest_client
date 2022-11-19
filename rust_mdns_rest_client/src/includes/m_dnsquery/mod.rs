// A mDNS query client.
//
// Note: there is no '.' at the end as the program adds ".local." automatically.
//
// Keeps listening for new events for 10 seconds.
#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use mdns_sd::{ ServiceDaemon, ServiceEvent };
use log::{ info };
use std::collections::hash_map::HashMap;

#[derive(Debug)]
pub struct MDNS {
    pub base_url: HashMap<String, String>,
}

impl MDNS {
    pub fn can_hold(&self, other: &MDNS) -> bool {
        self.base_url.len() > other.base_url.len()
    }
}

pub fn run_query(instance: &mut MDNS, mut service_type: String, scan_time: u64) {
    let mdns = ServiceDaemon::new().expect(
        "Failed to create daemon. Please install Bonjour on your system"
    );
    // Browse for a service type.
    service_type.push_str(".local.");
    let receiver = mdns
        .browse(&service_type)
        .expect("Failed to browse. Please install Bonjour on your system.");
    let now = std::time::Instant::now();
    // listen for event then stop the event loop after 5 seconds.
    while now.elapsed().as_secs() < scan_time {
        //let event = receiver.recv().expect("Failed to receive event");
        if let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    info!(
                        "At {:?}: Resolved a new service: {} IP: {:#?}:{:#?}",
                        now.elapsed(),
                        info.get_fullname(),
                        info.get_addresses(),
                        info.get_port()
                    );
                    // split the fullname by '.' and take the first element
                    let name = info.get_fullname().split('.').next().unwrap();
                    info!("Service name: {}", name);
                    // append name to 'http://' and '.local' to get the base url
                    let mut base_url = String::from("http://");
                    base_url.push_str(name);
                    base_url.push_str(".local");
                    info!("Base URL: {}", base_url);
                    // add the base url to the hashmap
                    instance.base_url.insert(name.to_string(), base_url);
                }
                other_event => {
                    info!("At {:?} : Received other event: {:?}", now.elapsed(), &other_event);
                }
            }
        }
    }
}

pub fn get_urls(instance: &MDNS) -> String {
    let mut urls = String::new();
    for (name, url) in &instance.base_url {
        //urls.push_str(name);
        //urls.push_str(": ");
        urls.push_str(url);
        //urls.push_str(""); // newline
    }
    urls
}

/* while let Ok(event) = receiver.recv() {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                info!(
                    "At {:?}: Resolved a new service: {} IP: {:?}:{:?}",
                    now.elapsed(),
                    info.get_fullname(),
                    info.get_addresses(),
                    info.get_port()
                );
                base_url.base_url.push(
                    format!("http://{}:{:?}", info.get_fullname(), info.get_port())
                );
            }
            other_event => {
                info!("At {:?} : Received other event: {:?}", now.elapsed(), &other_event);
            }
        }
    } */