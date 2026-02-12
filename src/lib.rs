//! # curl_smile
//!
//! Rust API for Keepsmile Bluetooth LE lights. Uses `btleplug` for cross-platform BLE support.
//!
//! ## Example
//!
//! This is the same example as `examples/use_curl_smile_demo.rs` in the repository: demos how to scan, connect,
//! update state, send commands, disconnect.
//!
//! ```no_run
//! use curl_smile::btle_communication::btle_api::{
//!     connect_to_btle_device, disconnect_from_btle_device, find_supported_devices,
//! };
//! use curl_smile::core::Intent::{Brightness, Rgb, SwitchOn};
//! use curl_smile::core::light_state::LightState;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let devices = find_supported_devices().await?;
//!
//!     for d in devices {
//!         connect_to_btle_device(&d).await?;
//!
//!         let mut state = LightState::new();
//!         state.update(SwitchOn(true));
//!         state.update(Brightness { brightness: 0x30 });
//!         state.update(Rgb {
//!             red: 0x0e,
//!             green: 0x00,
//!             blue: 0xaa,
//!         });
//!
//!         d.send_commands(&state).await?;
//!         disconnect_from_btle_device(&d).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Modules
//!
//! - `core`: high-level types.
//! - `btle_communication`: BLE scanning/connection.
//! - `hardware_abstraction_layer`: defines supported-devices and related GATT profile.
//! - `hardware_abstraction_layer/compiler`: converts state into command bytes for a target device.
//!
//! ## Supported devices
//!
//! See the repository README for the current supported-device list. Applications
//! should use `find_supported_devices()` rather than scanning for arbitrary peripherals.

pub mod core;
pub use core::{DeviceCommand, DeviceCompiler, Intent, LightState};

pub mod btle_communication;
pub mod hardware_abstraction_layer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_equality_works() {
        assert_eq!(
            Intent::Rgb {
                red: 255,
                green: 0,
                blue: 0
            },
            Intent::Rgb {
                red: 255,
                green: 0,
                blue: 0
            }
        );

        assert_ne!(
            Intent::Brightness { brightness: 10 },
            Intent::Brightness { brightness: 11 }
        );
    }

    #[test]
    fn light_state_starts_empty() {
        let s = LightState::new();
        assert_eq!(s.switch, None);
        assert_eq!(s.brightness, None);
        assert_eq!(s.rgb, None);
    }

    #[test]
    fn light_state_update_sets_fields() {
        let mut s = LightState::new();

        s.update(Intent::SwitchOn(true));
        s.update(Intent::Brightness { brightness: 100 });
        s.update(Intent::Rgb {
            red: 1,
            green: 2,
            blue: 3,
        });

        assert_eq!(s.switch, Some(true));
        assert_eq!(s.brightness, Some(100));
        assert_eq!(s.rgb, Some((1, 2, 3)));
    }
}
