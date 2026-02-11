#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    SwitchOn(bool),
    Brightness { brightness: u8 },
    ColorTemperature { ww: u16, cw: u16 },
    Rgb { red: u8, green: u8, blue: u8 },
    White { white: u8 },
    //Effect { effect: Effect },
    Speed(u8),
}
