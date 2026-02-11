use crate::core::{DeviceCommand, LightState};

pub trait DeviceCompiler {
    fn compile(&self, state: &LightState) -> Vec<DeviceCommand>;
}
