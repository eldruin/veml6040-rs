extern crate veml6040;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Transaction as I2cTransaction };
use veml6040::{ MeasurementMode, IntegrationTime };

mod common;
use common::{ DEVICE_ADDRESS, setup, Register };

fn get_write_expectation(config: u8) -> [I2cTransaction; 1] {
    [
        I2cTransaction::write(DEVICE_ADDRESS, vec![Register::CONFIG, config, 0])
    ]
}

macro_rules! config_test {
    ($name:ident, $method:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let expectations = get_write_expectation($expected);
            let mut dev = setup(&expectations);
            dev.$method().unwrap();
            dev.destroy().done();
        }
    };
}

config_test!(can_enable,  enable,  0);
config_test!(can_disable, disable, 1);

macro_rules! config_param_test {
    ($name:ident, $method:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let expectations = get_write_expectation($expected);
            let mut dev = setup(&expectations);
            dev.$method($input).unwrap();
            dev.destroy().done();
        }
    };
}

config_param_test!(can_set_mm_auto,   set_measurement_mode, MeasurementMode::Auto,   0);
config_param_test!(can_set_mm_manual, set_measurement_mode, MeasurementMode::Manual, 2);

config_test!(can_trigger_measurement, trigger_measurement, 4);

macro_rules! set_it_test {
    ($name:ident, $variant:ident, $expected:expr) => {
        config_param_test!(
            $name,
            set_integration_time,
            IntegrationTime::$variant,
            $expected
        );
    };
}

set_it_test!(can_set_it_40, _40ms, 0);
set_it_test!(can_set_it_80, _80ms, 1 << 4);
set_it_test!(can_set_it_160, _160ms, 2 << 4);
set_it_test!(can_set_it_320, _320ms, 3 << 4);
set_it_test!(can_set_it_640, _640ms, 4 << 4);
set_it_test!(can_set_it_1280, _1280ms, 5 << 4);
