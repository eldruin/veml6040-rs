extern crate veml6040;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Transaction as I2cTransaction };

mod common;
use common::{ DEVICE_ADDRESS, setup, Register };

fn get_expectation(register: u8) -> [I2cTransaction; 1] {
    [
        I2cTransaction::write_read(DEVICE_ADDRESS, vec![register], vec![0xCD, 0xAB])
    ]
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

read_test!(can_read_red_channel,   read_red_channel,   R_DATA);
read_test!(can_read_green_channel, read_green_channel, G_DATA);
read_test!(can_read_blue_channel,  read_blue_channel,  B_DATA);
read_test!(can_read_white_channel, read_white_channel, W_DATA);
