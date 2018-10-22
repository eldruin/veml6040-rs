extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Veml6040, DEVICE_ADDRESS, Register, BitFlags, MeasurementMode,
             Error };

impl<I2C, E> Veml6040<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Enable the sensor.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config & !BitFlags::SHUTDOWN)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config  = self.config;
        self.write_config(config | BitFlags::SHUTDOWN)
    }

    /// Set the measurement mode: `Auto`/`Manual`.
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            MeasurementMode::Auto   => self.write_config(config & !BitFlags::AF),
            MeasurementMode::Manual => self.write_config(config |  BitFlags::AF)
        }
    }

    /// Trigger a measurement when on `Manual` measurement mode.
    ///
    /// This is not necessary on `Auto` measurement mode.
    pub fn trigger_measurement(&mut self) -> Result<(), Error<E>> {
        // This bit is not stored to avoid unintended triggers.
        self.i2c
            .write(DEVICE_ADDRESS, &[Register::CONFIG, self.config | BitFlags::TRIG, 0])
            .map_err(Error::I2C)
    }

    fn write_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEVICE_ADDRESS, &[Register::CONFIG, config, 0])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}
