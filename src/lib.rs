//! This is a platform agnostic Rust driver for the VEML6040 RGBW color light
//! sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the sensor.
//! - Set the integration time.
//! - Set the measurement mode.
//! - Trigger a measurement when on manual mode.
//! - Read the red channel measurement.
//! - Read the green channel measurement.
//! - Read the blue channel measurement.
//! - Read the white channel measurement.
//! - Read measurement of all channels at once.
//!
//! ## The device
//!
//! VEML6040 color sensor senses red, green, blue, and white light and
//! incorporates photodiodes, amplifiers, and analog / digital circuits into a
//! single chip using CMOS process. With the color sensor applied, the
//! brightness, and color temperature of backlight can be adjusted base on
//! ambient light source that makes panel looks more comfortable for end
//! user's eyes. VEML6040's adoption of Filtron TM technology achieves the
//! closest ambient light spectral sensitivity to real human eye responses.
//! VEML6040 provides excellent temperature compensation capability for keeping
//! the output stable under changing temperature. VEML6040's function are
//! easily operated via the simple command format of I2C (SMBus compatible)
//! interface protocol. VEML6040's operating voltage ranges from 2.5 V to 3.6 V.
//!
//! Datasheet:
//! - [VEML6040](https://www.vishay.com/docs/84276/veml6040.pdf)
//!
//! Application note:
//! - [VEML6040 AN](https://www.vishay.com/docs/84331/designingveml6040.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! ### Enable and read the color measurement
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6040;
//!
//! use hal::I2cdev;
//! use veml6040::Veml6040;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6040::new(dev);
//! sensor.enable().unwrap();
//!
//! let red = sensor.read_red_channel().unwrap();
//! let green = sensor.read_green_channel().unwrap();
//! let blue = sensor.read_blue_channel().unwrap();
//! let white = sensor.read_white_channel().unwrap();
//!
//! println!("Measurements: red = {}, green = {}, blue = {}, white = {}",
//!          red, green, blue, white);
//! # }
//! ```
//!
//! ### Read all the channels at once
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6040;
//!
//! use hal::I2cdev;
//! use veml6040::Veml6040;
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6040::new(dev);
//! sensor.enable().unwrap();
//!
//! let measurement = sensor.read_all_channels().unwrap();
//!
//! println!("Measurements: red = {}, green = {}, blue = {}, white = {}",
//!          measurement.red, measurement.green, measurement.blue,
//!          measurement.white);
//! # }
//! ```
//!
//! ### Set the integration time to 320ms
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6040;
//!
//! use hal::I2cdev;
//! use veml6040::{ Veml6040, IntegrationTime };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6040::new(dev);
//! sensor.enable().unwrap();
//! sensor.set_integration_time(IntegrationTime::_320ms).unwrap();
//! # }
//! ```
//!
//! ### Set the measurement mode to manual and trigger a measurement
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate veml6040;
//!
//! use hal::I2cdev;
//! use veml6040::{ Veml6040, MeasurementMode };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Veml6040::new(dev);
//! sensor.enable().unwrap();
//! sensor.set_measurement_mode(MeasurementMode::Manual).unwrap();
//! sensor.trigger_measurement().unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Possible measurement modes
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementMode {
    /// Automatic mode.
    ///
    /// Measurements are made continuously. The actual cadence depends on
    /// the integration time.
    Auto,
    /// Manual mode.
    ///
    /// Measurements are only triggered manually. See `trigger_measurement()`.
    /// This is also called "force mode" or "ActiveForce" mode.
    Manual,
}

/// Integration time
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationTime {
    /// 40 ms
    _40ms,
    /// 80 ms
    _80ms,
    /// 160 ms
    _160ms,
    /// 320 ms
    _320ms,
    /// 640 ms
    _640ms,
    /// 1280 ms
    _1280ms,
}

/// Result of measurement of all channels
pub struct AllChannelMeasurement {
    /// Red channel measurement.
    pub red: u16,
    /// Green channel measurement.
    pub green: u16,
    /// Blue channel measurement.
    pub blue: u16,
    /// White channel measurement.
    pub white: u16,
}

const DEVICE_ADDRESS: u8 = 0x10;

struct Register;

impl Register {
    const CONFIG: u8 = 0x00;
    const R_DATA: u8 = 0x08;
    const G_DATA: u8 = 0x09;
    const B_DATA: u8 = 0x0A;
    const W_DATA: u8 = 0x0B;
}

struct BitFlags;

impl BitFlags {
    const SHUTDOWN: u8 = 0b0000_0001;
    const AF: u8 = 0b0000_0010;
    const TRIG: u8 = 0b0000_0100;
}

/// VEML6040 device driver.
#[derive(Debug, Default)]
pub struct Veml6040<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Configuration register status.
    config: u8,
}

impl<I2C, E> Veml6040<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the VEML6040 device.
    pub fn new(i2c: I2C) -> Self {
        Veml6040 { i2c, config: 0 }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

mod configuration;
mod reading;
