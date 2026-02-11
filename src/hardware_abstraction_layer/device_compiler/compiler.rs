use crate::core::{DeviceCommand, DeviceCompiler, LightState};
use crate::hardware_abstraction_layer::device::DeviceKind;

impl DeviceCompiler for DeviceKind {
    fn compile(&self, state: &LightState) -> Vec<DeviceCommand> {
        match self {
            DeviceKind::KS03New => ks03_compiler(state),
            DeviceKind::KS03Old => vec![],
        }
    }
}

fn ks03_compiler(state: &LightState) -> Vec<DeviceCommand> {
    let mut cmds = Vec::<DeviceCommand>::new();

    if let Some(on) = state.switch {
        let on_off: u8 = if on { 0xF0 } else { 0x0F };
        cmds.push(DeviceCommand {
            bytes: vec![0x5B, on_off, 0x00, 0xB5],
        });
    }

    if let Some((r, g, b)) = state.rgb {
        let r = scale_255_to_100(r);
        let g = scale_255_to_100(g);
        let b = scale_255_to_100(b);

        let is_rgb_hex = 0x01;
        let w = 0;
        let brightness = state.brightness.unwrap_or(0xFE);
        let speed = state.speed.unwrap_or(0x00);

        cmds.push(DeviceCommand {
            bytes: vec![0x5A, 0x00, is_rgb_hex, r, g, b, w, brightness, speed, 0xA5],
        });
    }

    cmds
}

fn scale_255_to_100(v: u8) -> u8 {
    ((v as u16 * 100) / 255) as u8
}
