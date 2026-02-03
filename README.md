# Design Pattern

## Core

Contains the data pipeline used for the API Logic

### Data Pipeline

`Intent`
: an enum, so it can be any type of defined command varients.

`LightState`
: a struct of diffrent intended commands (Intents) that define the latest state of the light.

`DeviceCompiler`
: can be thought of an abstract method or in rust terms, a trait. Where the compile method will implemented diffrently based on the device.

`DeviceCommand`
: A struct containing the generated payload (bytes) from the API compiler. The payload can change based on what device you are connected to. Produced by compiling LightState and consumed by the transport layer.


[Intent] -> [LightState] -> [DeviceCommand] -> [Transport]
Intent → LightState → [ DeviceCompiler ] → DeviceCommand → Transport
