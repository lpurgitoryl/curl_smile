use btleplug::api::Peripheral;
use curl_smile::btle_communication::btle_api::{
    connect_to_btle_device, disconnect_from_btle_device, find_supported_devices,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let devices = find_supported_devices().await?;
    for d in devices {
        println!(
            "Found supported: Device Kind {:?}, Device Name{}",
            d.kind, d.name
        );

        connect_to_btle_device(&d).await?;
        /*
             d.peripheral.discover_services().await?;
             println!("Discover peripheral {:?} services...", d.name);
             for service in d.peripheral.services() {
                 println!(
                     "Service UUID {}, primary: {}",
                     service.uuid, service.primary
                 );
                 for characteristic in service.characteristics {
                     println!("  {:?}", characteristic);
                 }
             }
        */
        disconnect_from_btle_device(&d).await?;
    }

    Ok(())
}
