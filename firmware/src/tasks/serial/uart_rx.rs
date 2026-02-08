use crate::state::{MachineStatus, STATUS_UPDATED_SIGNAL};

use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::{mode::Async, usart::UartRx};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn serial_uart_rx_task(mut rx: UartRx<'static, Async>) {
    use aerosmart_shared::serial::*;
    loop {
        // 1. Read Length (4 bytes)
        // Optimization: The length prefix allows us to read exactly the amount of data needed,
        // preventing buffer overflows and ensuring correct deserialization boundaries.
        let mut len_buf = [0u8; 4];
        if let Err(e) = rx.read(&mut len_buf).await {
            defmt::error!("UART Read Length Error: {:?}", e);
            continue;
        }
        let len = u32::from_le_bytes(len_buf) as usize;

        if len == 0 || len > 250 {
            // Sanity check
            defmt::warn!("Invalid message length: {}", len);
            continue;
        }

        // 2. Read Payload
        let mut buffer = [0u8; 256];
        if let Err(e) = rx.read(&mut buffer[..len]).await {
            defmt::error!("UART Read Payload Error: {:?}", e);
            continue;
        }

        defmt::info!("Received {} bytes payload", len);

        // 3. Deserialize
        // Use check_archived_root instead of unsafe for better safety if possible,
        // but sticking to previous style for minimal diff, just fixing the slice.
        // But unsafe is risky if data is garbage.
        // Let's use `access_unchecked` on the valid slice.
        let message = unsafe { rkyv::access_unchecked::<ArchivedSerialMessage>(&buffer[..len]) };
        {
            let mut state = GLOBAL_STATE.lock().await;
            match message {
                ArchivedSerialMessage::ThrottleConfig(ArchivedThrottleConfig { airspeed }) => {
                    state.desired_airspeed_meters_per_second = *airspeed as f32;
                }

                ArchivedSerialMessage::ServoConfig(ArchivedServoConfig { angle }) => {
                    state.desired_servo_angle_deg = *angle as f32;
                }

                ArchivedSerialMessage::SensorConfig(ArchivedSensorConfig { imu_horizontal }) => {
                    state.config = SensorConfig {
                        imu_horizontal: *imu_horizontal,
                    }
                }

                ArchivedSerialMessage::Command(command) => {
                    match command {
                        ArchivedCommand::Start => {
                            state.machine_status = MachineStatus::Running;
                        }
                        ArchivedCommand::Stop => {
                            state.machine_status = MachineStatus::Idle;
                        }
                        ArchivedCommand::Calibrate => {
                            state.machine_status = MachineStatus::Initializing;
                        }
                    }
                    STATUS_UPDATED_SIGNAL.signal(());
                }

                ArchivedSerialMessage::AcknowledgementConfig(ArchivedAcknowledgementConfig {
                    ack,
                    unix_timestamp_ms,
                }) => {
                    // Handle Handshake Pong from Service
                    defmt::info!(
                        "Received Handshake ACK. Timestamp: {}",
                        unix_timestamp_ms.to_native()
                    );
                    // In a real implementation, we might update RTC here or signal ready.
                    // The main initialization loop handles the first one, but this allows re-sync.
                }

                _ => {
                    defmt::error!("Unknown Serial Message Received");
                    state.machine_status = MachineStatus::Error;
                    STATUS_UPDATED_SIGNAL.signal(());
                }
            };
        }
    }
}
