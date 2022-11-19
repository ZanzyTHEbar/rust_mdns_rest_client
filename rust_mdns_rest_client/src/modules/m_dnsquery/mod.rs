//! A mdns query client.

#![allow(dead_code, unused_imports, unused_variables)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use mdns_sd::{ ServiceDaemon, ServiceEvent };
use log::{ info };
use std::collections::hash_map::HashMap;

/// A struct to hold the mDNS query results
/// - `base_url`: a hashmap of the base urls found
/// - `name`: a vector of the names of the devices found
#[derive(Debug)]
pub struct Mdns {
    pub base_url: HashMap<String, String>,
    pub name: Vec<String>,
}

/// A function to check if the mDNS struct can hold another instance
/// - `other`: a reference to the Mdns struct
/// ## Returns
/// - `bool`: true if the reference to the Mdns struct is smaller than the current struct instance
impl Mdns {
    pub fn can_hold(&self, other: &Mdns) -> bool {
        self.base_url.len() > other.base_url.len()
    }
}

/// Runs a mDNS query for X seconds
/// ## Arguments
/// - `mdns` A mutable reference to the Mdns struct
/// - `service_type` The service type to query for
/// - `scan_time` The number of seconds to query for
/// ## Example
/// ```
/// // Create a new Mdns struct
///let mut mdns: m_dnsquery::Mdns = m_dnsquery::Mdns {
///    base_url: HashMap::new(),
///    name: Vec::new(),
///};
/// // Run a mDNS query for 10 seconds
///let ref_mdns = &mut mdns;
///m_dnsquery::run_query(ref_mdns, String::from("_http._tcp"), 10);
/// ```
/// ## Notes
/// ***The service type should not have a '.' or a 'local' at the end.*** <br>
/// ***The program adds ".local." automatically.***
pub fn run_query(instance: &mut Mdns, mut service_type: String, scan_time: u64) {
    let mdns = ServiceDaemon::new().expect(
        "Failed to create daemon. Please install Bonjour on your system"
    );
    //* Browse for a service type.
    service_type.push_str(".local.");
    let receiver = mdns
        .browse(&service_type)
        .expect("Failed to browse. Please install Bonjour on your system.");
    let now = std::time::Instant::now();
    //* listen for event then stop the event loop after 5 seconds.
    // while let Ok(event) = receiver.recv() {}
    while now.elapsed().as_secs() < scan_time {
        //* let event = receiver.recv().expect("Failed to receive event");
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
                    //* split the fullname by '.' and take the first element
                    let name = info.get_fullname().split('.').next().unwrap();
                    info!("Service name: {}", name);
                    //* append name to 'http://' and '.local' to get the base url
                    let mut base_url = String::from("http://");
                    base_url.push_str(name);
                    base_url.push_str(".local");
                    info!("Base URL: {}", base_url);
                    //* add the base url to the hashmap
                    instance.base_url.insert(name.to_string(), base_url);
                    instance.name.push(name.to_string());
                }
                other_event => {
                    info!("At {:?} : Received other event: {:?}", now.elapsed(), &other_event);
                }
            }
        }
    }
}

/// Returns a map of the base urls found
/// ## Arguments
/// - `mdns` A mutable reference to the Mdns struct
/// ## Return
/// A map of all the base urls found for the service type
/// ## Example
/// ```
/// // Create a new Mdns struct
///let mut mdns: m_dnsquery::Mdns = m_dnsquery::Mdns {
///    base_url: HashMap::new(),
///    name: Vec::new(),
///};
/// // Run a query for 10 seconds
///let ref_mdns = &mut mdns;
///m_dnsquery::run_query(ref_mdns, String::from("_http._tcp"), 10);
/// // Get the base urls map
///let urls_map = m_dnsquery::get_url_map(ref_mdns); 
/// ```
pub fn get_url_map(instance: &mut Mdns) -> &mut HashMap<String, String> {
    &mut instance.base_url
}

/// Returns a vector of the base urls found
/// ## Arguments
/// - `mdns` A mutable reference to the Mdns struct
/// ## Return
/// A vector of all the urls found for the service type
/// ## Example
/// ```
/// // Create a new Mdns struct
/// let mut mdns: m_dnsquery::Mdns = m_dnsquery::Mdns {
///    base_url: HashMap::new(),
///   name: Vec::new(),
/// };
/// // Run a query for 10 seconds
/// let ref_mdns = &mut mdns;
/// m_dnsquery::run_query(ref_mdns, String::from("_http._tcp"), 10);
/// // Get the names vector
/// let vec = m_dnsquery::get_urls(ref_mdns);
/// ```
pub fn get_urls(instance: &Mdns) -> Vec<&String> {
    let mut urls: Vec<&String> = Vec::new();
    for (name, url) in &instance.base_url {
        urls.push(url);
    }
    urls
}
