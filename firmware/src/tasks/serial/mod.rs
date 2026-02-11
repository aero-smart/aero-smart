mod uart_rx;
mod uart_tx;

use aerosmart_shared::serial::{AcknowledgementData, ArchivedSerialMessage, SerialMessage};
use chrono::{Datelike, Timelike, Weekday};
use defmt::info;
use embassy_stm32::{
    mode::Async,
    rtc::{DateTime, DayOfWeek, Rtc},
    usart::Uart,
};
use embassy_time::Instant;
pub use uart_rx::serial_uart_rx_task;
pub use uart_tx::serial_uart_tx_task;

use crate::utils::send_message;

pub async fn serial_initialize<'a>(uart: &mut Uart<'a, Async>, rtc: &mut Rtc) {
    // Initialize the serial first to update the RTC timer and synchronize data
    let ack_packet = SerialMessage::AcknowledgementData(AcknowledgementData {
        time_elapsed_ms: Instant::now().as_millis(),
    });
    let (packet, length) = send_message(ack_packet).await;
    uart.write(&packet[..length]).await.unwrap();

    info!("Waiting for RTC synchronization message...");

    // Read Length Prefix (4 bytes)
    let mut len_buf = [0u8; 4];
    uart.read_until_idle(&mut len_buf).await.unwrap();
    let len = u32::from_le_bytes(len_buf) as usize;

    info!("Received handshake length prefix: {}", len);

    if len == 0 || len > 256 {
        defmt::error!("Invalid handshake message length: {}", len);
        return;
    }

    // Read Payload
    let mut buffer = [0u8; 256];
    uart.read_until_idle(&mut buffer).await.unwrap();

    info!("Received handshake payload: {} bytes", len);

    // Slice the received data to the actual length, and process it via `rotate_right`
    buffer[..len].rotate_right(1);

    // Deserialize
    let message = unsafe { rkyv::access_unchecked::<ArchivedSerialMessage>(&buffer[..len]) };
    match message {
        ArchivedSerialMessage::AcknowledgementConfig(config) => {
            let current = chrono::DateTime::from_timestamp_micros(
                config.unix_timestamp_ms.to_native() as i64,
            );
            if let Some(dt) = current {
                let new_datetime = DateTime::from(
                    dt.year() as u16,
                    dt.month() as u8,
                    dt.day() as u8,
                    match dt.weekday() {
                        Weekday::Mon => DayOfWeek::Monday,
                        Weekday::Tue => DayOfWeek::Tuesday,
                        Weekday::Wed => DayOfWeek::Wednesday,
                        Weekday::Thu => DayOfWeek::Thursday,
                        Weekday::Fri => DayOfWeek::Friday,
                        Weekday::Sat => DayOfWeek::Saturday,
                        Weekday::Sun => DayOfWeek::Sunday,
                    },
                    dt.hour() as u8,
                    dt.minute() as u8,
                    dt.second() as u8,
                    dt.nanosecond().wrapping_div(1000) as u32,
                )
                .unwrap();
                // rtc.set_datetime(new_datetime).ok();
                info!(
                    "RTC synchronized to UNIX timestamp: {}",
                    config.unix_timestamp_ms.to_native()
                );
            }
        }
        _ => {
            panic!("Unexpected message received during RTC sync");
        }
    }
}
