# Rust VEML6040 RGBW Color Sensor [![crates.io](https://img.shields.io/crates/v/veml6040.svg)](https://crates.io/crates/veml6040) [![Docs](https://docs.rs/veml6040/badge.svg)](https://docs.rs/veml6040) [![Build Status](https://travis-ci.org/eldruin/veml6040-rs.svg?branch=master)](https://travis-ci.org/eldruin/veml6040-rs)

This is a platform agnostic Rust driver for the VEML6040 RGBW color light
sensor, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the sensor.
- Set the integration time.
- Set the measurement mode.
- Trigger a measurement when on manual mode.

## The device
VEML6040 color sensor senses red, green, blue, and white light and
incorporates photodiodes, amplifiers, and analog / digital circuits into a
single chip using CMOS process. With the color sensor applied, the
brightness, and color temperature of backlight can be adjusted base on
ambient light source that makes panel looks more comfortable for end
user's eyes. VEML6040's adoption of Filtron TM technology achieves the
closest ambient light spectral sensitivity to real human eye responses.
VEML6040 provides excellent temperature compensation capability for keeping
the output stable under changing temperature. VEML6040's function are
easily operated via the simple command format of I2C (SMBus compatible)
interface protocol. VEML6040's operating voltage ranges from 2.5 V to 3.6 V.

Datasheet:
- [VEML6040](https://www.vishay.com/docs/84276/veml6040.pdf)

Application note:
- [VEML6040 AN](https://www.vishay.com/docs/84331/designingveml6040.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

