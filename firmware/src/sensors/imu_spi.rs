//! IMU SPI interface
//!
//! ICM-42688-P
//!
//! Poll @ 1 kHz at 1 MHz SPIuse defmt::info;
use crate::sensors::drivers::icm_42688_p::{AccelConfig0, GyroConfig0, IntSource0, PwrMgmt0};
use defmt::info;
use embassy_stm32::{
    gpio::Output,
    mode::Async,
    spi::{Error as SpiError, Spi},
};

pub struct ImuSpi<'a> {
    pub spi: Spi<'a, Async>,
    pub cs: Output<'a>,
}

#[derive(defmt::Format)]
pub enum ImuError {
    SpiError(SpiError),
    CsError,
}

#[derive(Debug, defmt::Format, Clone, Copy)]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

impl ImuData {
    pub fn new(
        accel_x: f32,
        accel_y: f32,
        accel_z: f32,
        gyro_x: f32,
        gyro_y: f32,
        gyro_z: f32,
    ) -> Self {
        Self {
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
        }
    }
}

impl<'a> ImuSpi<'a> {
    const PWR_MGMT0: u8 = 0x4E;
    const GYRO_CONFIG0: u8 = 0x4F;
    const ACCEL_CONFIG0: u8 = 0x50;
    const ACCEL_DATA_X1: u8 = 0x1F;
    const INT_SOURCE0: u8 = 0x66;

    pub fn new(spi: Spi<'a, Async>, cs: Output<'a>) -> Self {
        ImuSpi { spi, cs }
    }

    pub async fn poll(&mut self) -> Result<ImuData, ImuError> {
        self.cs.set_low();
        // perform SPI operations here
        let mut buffer = [0u8; { 1 + 6 * 2 }];
        let tx_buf = [0x80 | Self::ACCEL_DATA_X1];
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;
        self.spi
            .transfer_in_place(&mut buffer)
            .await
            .map_err(ImuError::SpiError)?;
        self.cs.set_high();

        info!("IMU Raw Data: {:?}", buffer);

        // TODO make it correct
        let accel_x = ((buffer[1] as i16) << 8 | (buffer[2] as i16)) as f32 / 16384.0;
        let accel_y = ((buffer[3] as i16) << 8 | (buffer[4] as i16)) as f32 / 16384.0;
        let accel_z = ((buffer[5] as i16) << 8 | (buffer[6] as i16)) as f32 / 16384.0;
        let gyro_x = ((buffer[7] as i16) << 8 | (buffer[8] as i16)) as f32 / 131.0;
        let gyro_y = ((buffer[9] as i16) << 8 | (buffer[10] as i16)) as f32 / 131.0;
        let gyro_z = ((buffer[11] as i16) << 8 | (buffer[12] as i16)) as f32 / 131.0;

        Ok(ImuData::new(
            accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z,
        ))
    }

    pub async fn init(&mut self) -> Result<(), ImuError> {
        self.cs.set_low();

        // Wake up the IMU
        let mut tx_buf = [Self::PWR_MGMT0, PwrMgmt0::enabled().to_byte()];
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        // Configure Gyroscope
        tx_buf = [Self::GYRO_CONFIG0, GyroConfig0::khz_1().to_byte()]; // Set full scale to ±250 dps
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        // Configure Accelerometer
        tx_buf = [Self::ACCEL_CONFIG0, AccelConfig0::khz_1().to_byte()]; // Set full scale to ±2g
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        tx_buf = [
            Self::INT_SOURCE0,
            IntSource0::to_byte(&[IntSource0::UiDrdy]),
        ];
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        self.cs.set_high();
        Ok(())
    }
}
