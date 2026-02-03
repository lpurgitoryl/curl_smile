// implementation of a last write wins accumalator

// For now, all you need to know is that <T> means that the Some variant of the Option enum can hold one piece of data of any type, 
// and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
use crate::intent::Intent;

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
    // constructor
    pub fn new() -> Self {
        Self::default()
    }

    // &mut seld becuase we want to change the state of the struct an own it
    // {...varibles name...} deoncstructs or spreads the parameters from the struct into vars of the same name
    // self.xyz then refrences the calling structs' fields and sets the option with some value based on the intent input
    // match works similarly to a case statement (more technically pattern matching)
    pub fn update(&mut self, intent: Intent) {
        match intent {
            Intent::Switch { on } => self.switch = Some(on),
            Intent::Brightness { brightness } => self.brightness = Some(brightness),
            Intent::ColorTemperature { ww, cw } => self.color_temperature = Some((ww, cw)),
            Intent::Rgb { red, green, blue } => self.rgb = Some((red, green, blue)),
            Intent::White { white } => self.white = Some(white),
        //    Intent::Effect { effect } => self.effect = Some(effect),
            Intent::Speed { speed } => self.speed = Some(speed),
        }
    }
}
