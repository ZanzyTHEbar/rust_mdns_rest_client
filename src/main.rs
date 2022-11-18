pub mod includes;

use log::info;
use includes::m_dnsquery;

fn main() {
    println!("Starting mDNS query");
    info!("Starting mDNS query");
    m_dnsquery::main().expect("Failed to run mDNS query");
}
