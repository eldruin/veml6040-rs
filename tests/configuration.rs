extern crate veml6040;
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Mock as I2cMock, Transaction as I2cTransaction };
use veml6040::{ Veml6040, MeasurementMode };

const DEVICE_ADDRESS: u8 = 0x10;

struct Register;

impl Register {
    const CONFIG : u8 = 0x00;
}

fn get_write_expectation(config: u8) -> [I2cTransaction; 1] {
    [
        I2cTransaction::write(DEVICE_ADDRESS, vec![Register::CONFIG, config, 0])
    ]
}

fn setup(expectations: &[I2cTransaction]) -> Veml6040<I2cMock> {
    let i2c = I2cMock::new(&expectations);
    Veml6040::new(i2c)
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
