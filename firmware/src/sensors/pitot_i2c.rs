//! Pitot tube and barometer I2C interface
//!
//! MS4525DO Pitot tube differential pressure sensor
//! BME280 Barometric pressure sensor
//!
//! Poll pitot @ 100 Hz and barometer @ 10 Hz over I2C
use super::drivers::{
    bme280::{AsyncBme280, Configuration, Oversampling, SensorMode},
    ms4525do::{ms4525do_pressure, ms4525do_temperature},
};
use aerosmart_shared::serial::BarometerData;
use defmt::debug;
use embassy_stm32::{
    i2c::{I2c, Master},
    mode::Async,
};
use embassy_time::{Delay, Instant, Timer};

#[derive(defmt::Format)]
pub enum AirspeedError {
    I2cError,
    InvalidData,
}

pub struct Airspeed {
    i2c: AsyncBme280<I2c<'static, Async, Master>, Delay>,

    /// MS4525DO calibration offset in Pascals
    ms4525do_offset_pa: f32,
}

impl Airspeed {
    pub const MS4525DO_ADDR: u8 = 0x28;
    pub const BME280_ADDR: u8 = 0x76;
    pub const BME280_CHIP_ID: u8 = 0x60;
    pub const BME280_REGISTER_PRESSUREDATA: u8 = 0xf7;
    pub const BME280_CTRL_MEAS: u8 = 0xf4;

    pub fn new(i2c: I2c<'static, Async, Master>) -> Self {
        let bme280 = AsyncBme280::new(i2c, Delay);

        Airspeed {
            i2c: bme280,
            ms4525do_offset_pa: 0.0,
        }
    }

    pub async fn read_pitot(&mut self) -> Result<(u8, f32, f32), AirspeedError> {
        use libm::fabsf as abs;

        let mut buf = [0u8; 4];

        debug!("Reading MS4525DO pitot sensor");
        self.i2c
            .i2c
            .read(Self::MS4525DO_ADDR, &mut buf)
            .await
            .map_err(|_| AirspeedError::I2cError)?;

        debug!("Raw pitot data: {:?}", buf);

        // Extract status (bits 7-6 of byte 0)
        let status = (buf[0] >> 6) & 0x03;

        // Extract pressure (14 bits)
        let pressure_raw = (((buf[0] & 0x3F) as u16) << 8) | (buf[1] as u16);

        // Extract temperature (11 bits)
        let temperature_raw = (((buf[2] as u16) << 3) | ((buf[3] >> 5) as u16)) & 0x07FF;

        let pressure_pa = ms4525do_pressure(pressure_raw);
        let temperature_c = ms4525do_temperature(temperature_raw);

        Ok((
            status,
            abs(pressure_pa) - self.ms4525do_offset_pa,
            temperature_c,
        ))
    }

    /// BME280 barometer reading
    pub async fn read_barometer(&mut self) -> Result<BarometerData, AirspeedError> {
        let pressure_pa = self
            .i2c
            .read_pressure()
            .await
            .map_err(|_| AirspeedError::I2cError)?
            .ok_or(AirspeedError::InvalidData)?;

        let temperature_c = self
            .i2c
            .read_temperature()
            .await
            .map_err(|_| AirspeedError::I2cError)?
            .ok_or(AirspeedError::InvalidData)?;

        let humidity_percent = self
            .i2c
            .read_humidity()
            .await
            .map_err(|_| AirspeedError::I2cError)?
            .ok_or(AirspeedError::InvalidData)?;

        Ok(BarometerData {
            pressure_pa,
            temperature_c,
            humidity_percent,
            time_elapsed_ms: Instant::now().as_millis(),
        })
    }

    pub async fn init(&mut self) -> Result<(), AirspeedError> {
        self.i2c.init().await.map_err(|_| AirspeedError::I2cError)?;

        self.i2c
            .set_sampling_configuration(
                Configuration::default()
                    .with_temperature_oversampling(Oversampling::Oversample4)
                    .with_pressure_oversampling(Oversampling::Oversample4)
                    .with_humidity_oversampling(Oversampling::Oversample4)
                    .with_sensor_mode(SensorMode::Normal),
            )
            .await
            .map_err(|_| AirspeedError::I2cError)?;

        self.calibrate().await?;

        Ok(())
    }

    pub async fn calibrate(&mut self) -> Result<(), AirspeedError> {
        for _attempt in 0..8 {
            let (_status, pressure_raw, _temperature_raw) = self.read_pitot().await?;
            self.ms4525do_offset_pa += pressure_raw;
            Timer::after_millis(100).await;
        }
        self.ms4525do_offset_pa /= 8.0;
        debug!("Calibrated MS4525DO offset: {} Pa", self.ms4525do_offset_pa);
        Ok(())
    }
}
