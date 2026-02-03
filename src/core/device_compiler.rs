use crate::core::{LightState, DeviceCommand};

pub trait DeviceCompiler {
    fn compile(&self, state: &LightState) -> Vec<DeviceCommand>;
}