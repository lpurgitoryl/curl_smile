use curl_smile::btle_communication::btle_api::{
    connect_to_btle_device, disconnect_from_btle_device, find_supported_devices,
};
use curl_smile::core::Intent::{Brightness, Rgb, Switch};
use curl_smile::core::light_state::LightState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let devices = find_supported_devices().await?;
    for d in devices {
        println!(
            "Found supported: Device Kind {:?}, Device Name{}",
            d.kind, d.name
        );

        connect_to_btle_device(&d).await?;
        let mut d_state = LightState::new();
        d_state.update(Switch { on: true });
        d_state.update(Brightness { brightness: 0x30 });
        d_state.update(Rgb {
            red: (0x0e),
            green: (0x0),
            blue: (0xaa),
        });

        d.send_commands(&d_state).await?;

        disconnect_from_btle_device(&d).await?;
    }

    Ok(())
}
