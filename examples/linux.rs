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
