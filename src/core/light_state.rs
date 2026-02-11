use crate::core::intent::Intent;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LightState {
    pub switch: Option<bool>,
    pub brightness: Option<u8>,
    pub color_temperature: Option<(u16, u16)>,
    pub rgb: Option<(u8, u8, u8)>,
    pub white: Option<u8>,
    // pub effect: Option<Effect>,
    pub speed: Option<u8>,
}

impl LightState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, intent: Intent) {
        match intent {
            Intent::SwitchOn(on) => self.switch = Some(on),
            Intent::Brightness { brightness } => self.brightness = Some(brightness),
            Intent::ColorTemperature { ww, cw } => self.color_temperature = Some((ww, cw)),
            Intent::Rgb { red, green, blue } => self.rgb = Some((red, green, blue)),
            Intent::White { white } => self.white = Some(white),
            //    Intent::Effect { effect } => self.effect = Some(effect),
            Intent::Speed(speed) => self.speed = Some(speed),
        }
    }
}
