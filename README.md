# curl_smile

Rust API for Keepsmile Bluetooth LE lights.
Uses `btleplug` under the hood.

## Usage

The example below scans for supported Bluetooth LE devices and tries to control any
which are supported.

Code below can be found in `examples\use_curl_smile_demo.rs`

```rust
use curl_smile::btle_communication::btle_api::{
    connect_to_btle_device, disconnect_from_btle_device, find_supported_devices,
};
use curl_smile::core::Intent::{Brightness, Rgb, SwitchOn};
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
        d_state.update(SwitchOn(true));
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
```

## Supported Devices

| Device                           | Bluetooth Name | Support |
| -------------------------------- | -------------- | ------- |
| Keepsmile Led Strip Lights (New) | KS03~XXXX      | Yes     |

## Acknowledgments

This is a simplified Rust port of this Python package.
Thank you to [@themooer1](https://github.com/themooer1) for the great repo!

- [cheshire](https://github.com/themooer1/cheshire)
