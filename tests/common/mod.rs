extern crate veml6040;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Mock as I2cMock, Transaction as I2cTransaction };
use veml6040::Veml6040;

pub const DEVICE_ADDRESS: u8 = 0x10;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const CONFIG : u8 = 0x00;
    pub const R_DATA : u8 = 0x08;
    pub const G_DATA : u8 = 0x09;
    pub const B_DATA : u8 = 0x0A;
    pub const W_DATA : u8 = 0x0B;
}

pub fn setup(expectations: &[I2cTransaction]) -> Veml6040<I2cMock> {
    let i2c = I2cMock::new(&expectations);
    Veml6040::new(i2c)
}
