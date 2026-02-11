use std::time::Duration;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use tokio::time;

use crate::hardware_abstraction_layer::device::{DeviceKind, DeviceProfile, SupportedDevice};

pub async fn find_supported_devices() -> anyhow::Result<Vec<SupportedDevice>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    let mut supported: Vec<SupportedDevice> = Vec::new();

    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;

        let peripherals = adapter.peripherals().await?;

        for peripheral in peripherals.iter() {
            let properties = peripheral.properties().await?;
            let local_name = properties
                .unwrap()
                .local_name
                .unwrap_or(String::from("(peripheral name unknown)"));

            if let Some(kind) = DeviceKind::from_name(&local_name) {
                supported.push(SupportedDevice {
                    name: local_name,
                    kind,
                    peripheral: peripheral.clone(),
                    device_profile: DeviceProfile::from_kind(kind),
                });
            }
        }
    }

    Ok(supported)
}

pub async fn connect_to_btle_device(device: &SupportedDevice) -> anyhow::Result<()> {
    let is_connected = device.peripheral.is_connected().await?;

    if !is_connected {
        if let Err(err) = device.peripheral.connect().await {
            eprintln!("Error connecting to peripheral, skipping: {}", err);
        }
    }

    time::sleep(Duration::from_secs(10)).await;
    let is_connected = device.peripheral.is_connected().await?;
    println!(
        "Now connected ({:?}) to peripheral {:?}...",
        is_connected, device.name
    );
    Ok(())
}

pub async fn disconnect_from_btle_device(device: &SupportedDevice) -> anyhow::Result<()> {
    let is_connected = device.peripheral.is_connected().await?;

    if is_connected {
        println!("Disconnecting from peripheral {:?}...", device.name);
        device
            .peripheral
            .disconnect()
            .await
            .expect("Error disconnecting from BLE peripheral");
    }
    Ok(())
}

pub async fn send() {}
