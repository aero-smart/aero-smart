#![allow(dead_code)]
use aerosmart_shared::serial::ImuData;
use embassy_stm32::{
    gpio::Output,
    mode::Async,
    spi::{Error as SpiError, Spi},
};

pub struct ImuSpi<'a> {
    pub spi: Spi<'a, Async>,
    pub cs: Output<'a>,
}

pub enum ImuError {
    SpiError(SpiError),
    CsError,
}

impl<'a> ImuSpi<'a> {
    const WHO_AM_I: u8 = 0x75;
    const PWR_MGMT0: u8 = 0x4E;
    const GYRO_CONFIG0: u8 = 0x4F;
    const ACCEL_CONFIG0: u8 = 0x50;
    const INT_STATUS: u8 = 0x2D;
    const ACCEL_DATA_X1: u8 = 0x1F;

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
        let mut tx_buf = [Self::PWR_MGMT0 & 0x7F, 0x01];
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        // Configure Gyroscope
        tx_buf = [Self::GYRO_CONFIG0 & 0x7F, 0x00]; // Set full scale to ±250 dps
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        // Configure Accelerometer
        tx_buf = [Self::ACCEL_CONFIG0 & 0x7F, 0x00]; // Set full scale to ±2g
        self.spi.write(&tx_buf).await.map_err(ImuError::SpiError)?;

        self.cs.set_high();
        Ok(())
    }
}
