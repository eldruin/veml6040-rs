
extern crate embedded_hal as hal;
use hal::blocking::i2c;
use super::{ Veml6040, DEVICE_ADDRESS, Register, BitFlags, Error };

impl<I2C, E> Veml6040<I2C>
where
    I2C: i2c::WriteRead<Error = E>
{
    /// Read the red channel measurement data.
    pub fn read_red_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::R_DATA)
    }

    /// Read the green channel measurement data.
    pub fn read_green_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::G_DATA)
    }

    /// Read the blue channel measurement data.
    pub fn read_blue_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::B_DATA)
    }
 
    /// Read the white channel measurement data.
    pub fn read_white_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::W_DATA)
    }

    fn read_channel(&mut self, first_register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[first_register], &mut data)
            .map_err(Error::I2C)?;
        Ok((data[1] as u16) << 8 | data[0] as u16)
    }
}
