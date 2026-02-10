pub mod core;
pub use core::{DeviceCommand, DeviceCompiler, Intent, LightState};

pub mod btle_communication;
pub mod hardware_abstraction_layer;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

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

        s.update(Intent::Switch { on: true });
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
