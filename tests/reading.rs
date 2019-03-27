extern crate embedded_hal_mock as hal;
extern crate veml6040;
use hal::i2c::Transaction as I2cTransaction;

mod common;
use common::{setup, Register, DEVICE_ADDRESS};

fn get_expectation(register: u8) -> [I2cTransaction; 1] {
    [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![register],
        vec![0xCD, 0xAB],
    )]
}

macro_rules! read_test {
    ($name:ident, $method:ident, $register:ident) => {
        #[test]
        fn $name() {
            let expectations = get_expectation(Register::$register);
            let mut dev = setup(&expectations);
            let data = dev.$method().unwrap();
            assert_eq!(0xABCD, data);
            dev.destroy().done();
        }
    };
}

read_test!(can_read_red_channel, read_red_channel, R_DATA);
read_test!(can_read_green_channel, read_green_channel, G_DATA);
read_test!(can_read_blue_channel, read_blue_channel, B_DATA);
read_test!(can_read_white_channel, read_white_channel, W_DATA);

#[test]
fn can_read_all_channels() {
    let expectations = [I2cTransaction::write_read(
        DEVICE_ADDRESS,
        vec![Register::R_DATA],
        vec![0x23, 0x01, 0x67, 0x45, 0xAB, 0x89, 0xEF, 0xCD],
    )];
    let mut dev = setup(&expectations);
    let measurement = dev.read_all_channels().unwrap();
    assert_eq!(0x0123, measurement.red);
    assert_eq!(0x4567, measurement.green);
    assert_eq!(0x89AB, measurement.blue);
    assert_eq!(0xCDEF, measurement.white);
    dev.destroy().done();
}
