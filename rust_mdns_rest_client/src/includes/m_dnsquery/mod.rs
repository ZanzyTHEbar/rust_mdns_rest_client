use std::{ time::Duration };

const SERVICE_NAME: &'static str = "_waterchamber._tcp.local";

#[tokio::main]
async fn main() -> Result<(), async_zeroconf::ZeroconfError> {
    let mut browser = async_zeroconf::ServiceBrowserBuilder::new(SERVICE_NAME);
    let mut services = browser
        .timeout(tokio::time::Duration::from_secs(2))
        .browse()?;

    while let Some(Ok(v)) = services.recv().await {
        println!("Service = {}", v);
    }
    Ok(())
}