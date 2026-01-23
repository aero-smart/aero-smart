#![allow(dead_code)]
/// Pitot tube and barometer I2C interface
///
/// MS4525DO Pitot tube differential pressure sensor
/// BME280 Barometric pressure sensor
///
/// Poll pitot @ 100 Hz and barometer @ 10 Hz over I2C
use aerosmart_shared::serial::BarometerData;
use embassy_stm32::{
    i2c::{Error, I2c, Master},
    mode::Async,
};
use num_traits::float::Float;

#[derive(defmt::Format)]
pub enum AirspeedError {
    I2cError(Error),
    InvalidData,
}

pub struct Airspeed<'a> {
    i2c: I2c<'a, Async, Master>,
}

impl<'a> Airspeed<'a> {
    pub const MS4525DO_ADDR: u8 = 0x28;
    pub const BME280_ADDR: u8 = 0x76;
    pub const BME280_CHIP_ID: u8 = 0x60;
    pub const BME280_REGISTER_PRESSUREDATA: u8 = 0xf7;

    pub fn new(i2c: I2c<'a, Async, Master>) -> Self {
        Self { i2c }
    }

    pub async fn read_raw_pitot(&mut self) -> Result<(u16, u16, u16), AirspeedError> {
        let mut buf = [0u8; 4];
        self.i2c
            .read(Self::MS4525DO_ADDR, &mut buf)
            .await
            .map_err(AirspeedError::I2cError)?;

        // Extract status (bits 7-6 of byte 0)
        let status = (buf[0] >> 6) & 0x03;

        // Extract pressure (14 bits)
        let pressure_raw = (((buf[0] & 0x3F) as u16) << 8) | (buf[1] as u16);

        // Extract temperature (11 bits)
        let temperature_raw = (((buf[2] as u16) << 3) | ((buf[3] >> 5) as u16)) & 0x07FF;

        Ok((status as u16, pressure_raw, temperature_raw))
    }

    /// BME280 barometer reading
    pub async fn read_barometer(&mut self) -> Result<BarometerData, AirspeedError> {
        let mut buf = [0u8; 6];
        self.i2c
            .write_read(
                Self::BME280_ADDR,
                &[Self::BME280_REGISTER_PRESSUREDATA],
                &mut buf,
            )
            .await
            .map_err(AirspeedError::I2cError)?;

        let pressure_raw: u32 =
            ((buf[0] as u32) << 12) | ((buf[1] as u32) << 4) | ((buf[2] as u32) >> 4);
        let temperature_raw: u32 =
            ((buf[3] as u32) << 12) | ((buf[4] as u32) << 4) | ((buf[5] as u32) >> 4);
        let humidity_raw: u16 = ((buf[6] as u16) << 8) | (buf[7] as u16);

        // Convert raw values to physical units
        // Don't use `convert_pressure` and `convert_temperature` here because they are for MS4525DO
        // TODO: Implement BME280 compensation formulas using calibration data
        let pressure_pa = (pressure_raw as f32) / 256.0; // Example conversion
        let temperature_c = (temperature_raw as f32) / 100.0; // Example conversion
        let humidity_percent = (humidity_raw as f32) / 65536.0 * 100.0;

        Ok(BarometerData {
            pressure_pa,
            temperature_c,
            humidity_percent,
        })
    }

    #[inline]
    fn convert_pressure(raw: u16) -> f32 {
        // MS4525DO: 1 PSI differential pressure sensor
        // 1 PSI = 6894.76 Pa
        const P_MIN_PSI: f32 = -1.0;
        const P_MAX_PSI: f32 = 1.0;
        const PSI_TO_PA: f32 = 6894.76;

        const P_MIN_PA: f32 = P_MIN_PSI * PSI_TO_PA; // -6894.76 Pa
        const P_MAX_PA: f32 = P_MAX_PSI * PSI_TO_PA; // 6894.76 Pa

        const OUTPUT_MIN: f32 = 1638.0; // 10% of 2^14
        const OUTPUT_MAX: f32 = 14745.0; // 90% of 2^14

        let raw_f = raw as f32;
        (raw_f - OUTPUT_MIN) * (P_MAX_PA - P_MIN_PA) / (OUTPUT_MAX - OUTPUT_MIN) + P_MIN_PA
    }

    #[inline]
    fn convert_temperature(raw: u16) -> f32 {
        // Temperature formula from datasheet
        // Temp (°C) = (raw * 200 / 2047) - 50
        (raw as f32 * 200.0 / 2047.0) - 50.0
    }

    #[inline]
    fn calculate_air_density(temp_celsius: f32) -> f32 {
        // Simplified air density calculation at sea level
        // ρ = P / (R × T)
        // Where:
        //   P = atmospheric pressure (Pa) = 101325 Pa at sea level
        //   R = specific gas constant for dry air = 287.05 J/(kg·K)
        //   T = absolute temperature (K)

        const P_ATM: f32 = 101325.0; // Pa
        const R_AIR: f32 = 287.05; // J/(kg·K)

        let temp_kelvin = temp_celsius + 273.15;
        P_ATM / (R_AIR * temp_kelvin)
    }

    #[inline]
    fn calculate_airspeed(pressure_pa: f32, air_density: f32) -> f32 {
        // Bernoulli's equation: v = sqrt(2 * ΔP / ρ)
        if pressure_pa.abs() < f32::EPSILON {
            0.0
        } else {
            let velocity = (2.0 * pressure_pa / air_density).sqrt();
            if pressure_pa < 0.0 {
                -velocity
            } else {
                velocity
            }
        }
    }
}
