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

    pub accel_config: AccelConfig0,
    pub gyro_config: GyroConfig0,
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

    pub fn new(
        spi: Spi<'a, Async>,
        cs: Output<'a>,
        accel_config: AccelConfig0,
        gyro_config: GyroConfig0,
    ) -> Self {
        ImuSpi {
            spi,
            cs,
            accel_config,
            gyro_config,
        }
    }

    pub async fn poll(&mut self) -> Result<ImuData, ImuError> {
        const RAW_MAX: f32 = 32768.0;
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

        let accel_x = i16::from_be_bytes([buffer[1], buffer[2]]) as f32 / RAW_MAX
            * self.accel_config.fs_sel.scale_factor();
        let accel_y = i16::from_be_bytes([buffer[3], buffer[4]]) as f32 / RAW_MAX
            * self.accel_config.fs_sel.scale_factor();
        let accel_z = i16::from_be_bytes([buffer[5], buffer[6]]) as f32 / RAW_MAX
            * self.accel_config.fs_sel.scale_factor();
        let gyro_x = i16::from_be_bytes([buffer[7], buffer[8]]) as f32 / RAW_MAX
            * self.gyro_config.fs_sel.scale_factor();
        let gyro_y = i16::from_be_bytes([buffer[9], buffer[10]]) as f32 / RAW_MAX
            * self.gyro_config.fs_sel.scale_factor();
        let gyro_z = i16::from_be_bytes([buffer[11], buffer[12]]) as f32 / RAW_MAX
            * self.gyro_config.fs_sel.scale_factor();

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
        tx_buf = [Self::GYRO_CONFIG0, self.gyro_config.to_byte()]; // Set full scale to ±250 dps
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        // Configure Accelerometer
        tx_buf = [Self::ACCEL_CONFIG0, self.accel_config.to_byte()]; // Set full scale to ±2g
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
