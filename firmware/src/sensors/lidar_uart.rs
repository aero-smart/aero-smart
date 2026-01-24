#![allow(dead_code)]
//! TFmini-S LiDAR sensor over UART interface.
//! - Baud Rate: 115200
//! - Data Frame: 9 bytes

use aerosmart_shared::serial::LidarData;
use embassy_stm32::{mode::Async, usart::Uart};

pub struct LidarUart<'a> {
    pub uart: Uart<'a, Async>,
}

#[derive(defmt::Format)]
pub enum LidarError {
    UartError,
    InvalidData,
}

impl<'a> LidarUart<'a> {
    pub fn new(uart: Uart<'a, Async>) -> Self {
        LidarUart { uart }
    }

    pub async fn poll(&mut self) -> Result<LidarData, LidarError> {
        let mut buffer = [0u8; 9];
        self.uart
            .read(&mut buffer)
            .await
            .map_err(|_| LidarError::UartError)?;

        // Validate frame header
        if buffer[0] != 0x59 || buffer[1] != 0x59 {
            return Err(LidarError::InvalidData);
        }

        let distance_cm = u16::from_le_bytes([buffer[2], buffer[3]]);
        let signal_strength = u16::from_le_bytes([buffer[4], buffer[5]]);

        Ok(LidarData {
            distance_cm,
            signal_strength,
        })
    }
}
