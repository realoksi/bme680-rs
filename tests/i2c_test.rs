mod common;
use common::I2c;

use bme680_rs::BME680;

#[test]
fn i2c_test() {
    let layer = I2c::new();

    let mut bme680 = BME680::new(layer);

    assert_eq!(bme680.read_chip_id().unwrap(), 0x61);
}
