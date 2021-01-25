# Rust VEML6040 RGBW Color Sensor Driver

[![crates.io](https://img.shields.io/crates/v/veml6040.svg)](https://crates.io/crates/veml6040)
[![Docs](https://docs.rs/veml6040/badge.svg)](https://docs.rs/veml6040)
[![Build Status](https://github.com/eldruin/veml6040-rs/workflows/Build/badge.svg)](https://github.com/eldruin/veml6040-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/veml6040-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/veml6040-rs?branch=master)

This is a platform agnostic Rust driver for the VEML6040 RGBW color light
sensor, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the sensor.
- Set the integration time.
- Set the measurement mode.
- Trigger a measurement when on manual mode.
- Read the red channel measurement.
- Read the green channel measurement.
- Read the blue channel measurement.
- Read the white channel measurement.
- Read measurement of all channels at once.

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

Datasheet: [VEML6040](https://www.vishay.com/docs/84276/veml6040.pdf)

Application note: [VEML6040 AN](https://www.vishay.com/docs/84331/designingveml6040.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use veml6040::Veml6040;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Veml6040::new(dev);
    sensor.enable().unwrap();

    let red = sensor.read_red_channel().unwrap();
    let green = sensor.read_green_channel().unwrap();
    let blue = sensor.read_blue_channel().unwrap();
    let white = sensor.read_white_channel().unwrap();

    println!(
        "Measurements: R: {}, G: {}, B: {}, W: {}",
        red, green, blue, white
    );
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/veml6040-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
   
at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

