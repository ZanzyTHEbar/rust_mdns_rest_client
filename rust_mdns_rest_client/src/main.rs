pub mod includes;

use log::info;
use includes::m_dnsquery;

fn main() {
    env_logger::init();
    info!("This is a debug statement");
    m_dnsquery::main().expect("Error");
}