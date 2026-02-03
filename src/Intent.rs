/* 
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
'enums give you a way of saying a value is one of a possible set of values'

Here we define an enum Command where it can contain different kinds (variants) of said command
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    Switch { on: bool },
    Brightness { brightness: u8 },
    ColorTemperature { ww: u16, cw: u16 },
    Rgb { red: u8, green: u8, blue: u8 },
    White { white: u8 },
    Effect { effect: Effect },
    Speed { speed: u8 },
}