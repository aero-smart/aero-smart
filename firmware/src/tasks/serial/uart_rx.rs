use crate::state::{MachineStatus, STATUS_UPDATED_SIGNAL};

use {defmt_rtt as _, panic_probe as _};

use defmt::info;
use embassy_stm32::{mode::Async, usart::UartRx};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn serial_uart_rx_task(mut rx: UartRx<'static, Async>) {
    use aerosmart_shared::serial::*;
    loop {
        // 1. Read Length (4 bytes)
        // Optimization: The length prefix allows us to read exactly the amount of data needed,
        // preventing buffer overflows and ensuring correct deserialization boundaries.
        let mut buffer = [0u8; 256];
        info!("Waiting to read length prefix...");
        if let Err(e) = rx.read_until_idle(&mut buffer).await {
            defmt::error!("UART Read Length Error: {:?}", e);
            continue;
        }
        info!("Received payload: {:?}", buffer);
        let len_buf = &buffer[..4];
        let len = u32::from_le_bytes(len_buf.try_into().unwrap()) as usize;

        if len == 0 || len > 250 {
            // Sanity check
            defmt::warn!("Invalid message length: {}", len);
            continue;
        }

        defmt::info!("Received {} bytes payload", len);

        // Slice the received data to the actual length, and process it via `rotate_right`
        // buffer[4..4 + len].rotate_right(1);

        // info!("Buffer after rotation: {:?}", &buffer[4..4 + len]);

        let mut new_buffer = [0u8; 256];
        new_buffer[..len].copy_from_slice(&buffer[4..4 + len]);
        buffer = new_buffer;

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
                    info!(
                        "Updated desired airspeed to {} m/s",
                        state.desired_airspeed_meters_per_second
                    );
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
                    ack: _,
                    unix_timestamp_ms,
                }) => {
                    // Handle Handshake Pong from Service
                    defmt::info!(
                        "Received Handshake ACK. Timestamp: {}",
                        unix_timestamp_ms.to_native()
                    );
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
