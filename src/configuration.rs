extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Veml6040, DEVICE_ADDRESS, Register, BitFlags, MeasurementMode,
             IntegrationTime, Error };

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

    /// Set the integration time.
    pub fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<E>> {
        const IT_BITS: u8 = 0b0111_0000;
        let config = self.config & !IT_BITS;
        match it {
            IntegrationTime::_40ms   => self.write_config(config | 0b0000_0000),
            IntegrationTime::_80ms   => self.write_config(config | 0b0001_0000),
            IntegrationTime::_160ms  => self.write_config(config | 0b0010_0000),
            IntegrationTime::_320ms  => self.write_config(config | 0b0011_0000),
            IntegrationTime::_640ms  => self.write_config(config | 0b0100_0000),
            IntegrationTime::_1280ms => self.write_config(config | 0b0101_0000),
        }
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
