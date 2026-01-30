use embassy_stm32::{
    i2c::{I2c, Master},
    mode::Async,
};

/// ADS1115 driver over I2C
///
/// According to the plan, we use ADS1115 for reading the data from XGZP6847A and MPXV7002 (possible).
///
/// However, since I don't want to use ADC peripheral of STM32 right now, in addition to the truth that those sensors' output voltage is 0.5 V to 4.5 V, which is beyond the range of ADC peripheral (0 V to 3.3 V),
/// I decided to use ADS1115 for reading the data from both sensors.
pub struct AdcI2c<'a> {
    pub i2c: I2c<'a, Async, Master>,
}

impl<'a> AdcI2c<'a> {
    const ADDRESS: u8 = 0b1001000;

    pub fn new(i2c: I2c<'a, Async, Master>) -> Self {
        Self { i2c }
    }

    pub fn init(&mut self) {}
}
