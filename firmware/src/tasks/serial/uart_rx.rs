use crate::state::{MachineStatus, STATUS_UPDATED_SIGNAL};

use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::{mode::Async, usart::UartRx};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn serial_uart_rx_task(mut rx: UartRx<'static, Async>) {
    use aerosmart_shared::serial::*;
    loop {
        let mut buffer = [0u8; 256];
        match rx.read(&mut buffer).await {
            Ok(len) => {
                defmt::info!("Received {} bytes over UART", len);
            }
            Err(e) => {
                defmt::error!("UART Read Error: {:?}", e);
                continue;
            }
        }
        let message = unsafe { rkyv::access_unchecked::<ArchivedSerialMessage>(&buffer) };
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

                _ => {
                    defmt::error!("Unknown Serial Message Received");
                    state.machine_status = MachineStatus::Error;
                    STATUS_UPDATED_SIGNAL.signal(());
                }
            };
        }
    }
}
