# to rust or not rust

TLDR; I'm learning rust as im doing this port of `cheshire` the python module. These are my notes

## rust is weird but also familar

- Very OOP Like
- the use of pub is similar to c++
- the use of modules is kinda spicy compared to say TS or Python
- very stricly typed, but I like how the compiler and linter will yell at you for everything. It makes me really ponder my choices.

### Notes

`Intent`
: an enum, so it can be any type of defined command varients.

    https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    "enums give you a way of saying a value is one of a possible set of values"

`LightState`
: a struct of diffrent intended commands (Intents) that define the latest state of the light.

    same concept as structs in other languages. Rust is technically OOP but no how c++ is. You can define struct (the shape of the data of your object) and with impl then define methods/functions to which get/set/do/ etc with said data.

`DeviceCompiler`
: can be thought of an abstract method or in rust terms, a trait. Where the compile method will implemented diffrently based on the device.

    Since there isn't direct inheritance, this trait seems to be like pointer->func() type vibe. Boiler plate of the method. "Their specific purpose is to allow abstraction across common behavior"

`DeviceCommand`
: A struct containing the generated payload (bytes) from the API compiler. The payload can change based on what device you are connected to. Produced by compiling LightState and consumed by the transport layer.

`Intent` feeds into the `LightState` where it calls the `DeviceCompiler` which generates the correct `DeviceCommand`. This is then used by the `BLETransmitter`

### Questions

- define the headers above enums, do structs have that too ? why ?
- there has to be benifits to not directly including method implemenation with the structs ? right now it just seems like a bit of extra steps.
