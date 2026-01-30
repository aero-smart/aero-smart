use aerosmart_shared::serial::AnalogPressureSensorData;
use embassy_stm32::{
    exti::ExtiInput,
    i2c::{I2c, Master},
    mode::Async,
};

use crate::{
    sensors::drivers::ads1115::Ads1115Config,
    utils::pressure_adc::{option_arr_to_messsage, voltage_v_to_pressure_pa},
};

/// ADS1115 driver over I2C
///
/// According to the plan, we use ADS1115 for reading the data from XGZP6847A and MPXV7002 (possible).
///
/// However, since I don't want to use ADC peripheral of STM32 right now, in addition to the truth that those sensors' output voltage is 0.5 V to 4.5 V, which is beyond the range of ADC peripheral (0 V to 3.3 V),
/// I decided to use ADS1115 for reading the data from both sensors.
pub struct AdcI2c<'a> {
    pub i2c: I2c<'a, Async, Master>,
    pub channels: [Option<AdcConnection>; 4],
    pub drdy_pin: ExtiInput<'a>,
}

#[derive(Debug, Copy, Clone, PartialEq, defmt::Format)]
pub enum AdcI2cError {
    I2cError,
    InvalidChannel,
}

/// The type of connection between the ADC and the sensor.
#[derive(Debug, Copy, Clone, PartialEq, defmt::Format)]
pub enum AdcConnection {
    Mpxv7002,
    Xgzp6847aPa2500,
    Xgzp6847aPa3000,
}

impl AdcConnection {
    pub fn aquire_data_pa(&self, voltage_v: f32) -> f32 {
        voltage_v_to_pressure_pa(
            voltage_v,
            match self {
                AdcConnection::Mpxv7002 => (-2_000.0, 2_000.0),
                AdcConnection::Xgzp6847aPa2500 => (0.0, 2500.0),
                AdcConnection::Xgzp6847aPa3000 => (0.0, 3000.0),
            },
        )
    }
}

impl<'a> AdcI2c<'a> {
    const ADDRESS: u8 = 0b1001000;
    const CONFIG_ADDRESS: u8 = 0x01;
    const READ_ADDRESS: u8 = 0x00;

    pub fn new(
        i2c: I2c<'a, Async, Master>,
        ch1: Option<AdcConnection>,
        ch2: Option<AdcConnection>,
        ch3: Option<AdcConnection>,
        ch4: Option<AdcConnection>,
        drdy_pin: ExtiInput<'a>,
    ) -> Self {
        Self {
            i2c,
            channels: [ch1, ch2, ch3, ch4],
            drdy_pin,
        }
    }

    pub async fn poll_chan(&mut self, channel: usize) -> Result<f32, AdcI2cError> {
        if channel >= 4 {
            return Err(AdcI2cError::InvalidChannel);
        }
        let config = Ads1115Config::chan(channel);
        let conf = config.to_bytes();
        let mut data = [0u8; 3];
        data[0] = Self::CONFIG_ADDRESS;
        data[1..].copy_from_slice(&conf);
        self.i2c
            .write(Self::ADDRESS, &data)
            .await
            .map_err(|_| AdcI2cError::I2cError)?;
        // Wait for conversion to complete
        self.drdy_pin.wait_for_falling_edge().await;
        // Read conversion result
        let mut read_buf = [0u8; 2];
        self.i2c
            .write_read(Self::ADDRESS, &[Self::READ_ADDRESS], &mut read_buf)
            .await
            .map_err(|_| AdcI2cError::I2cError)?;
        let raw_value = i16::from_be_bytes(read_buf);
        let adc_volt_raw = config.pga.to_voltage(raw_value);
        self.channels
            .get(channel)
            .and_then(|conn| *conn)
            .map(|connection| Ok(connection.aquire_data_pa(adc_volt_raw)))
            .ok_or(AdcI2cError::InvalidChannel)?
    }

    pub async fn poll_all(&mut self) -> Result<AnalogPressureSensorData, AdcI2cError> {
        let mut results = [None; 4];
        for channel in 0..4 {
            if self.channels[channel].is_some() {
                let pressure_pa = self.poll_chan(channel).await?;
                results[channel] = Some(pressure_pa);
            }
        }
        Ok(option_arr_to_messsage(results))
    }
}
