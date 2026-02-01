use crate::utils::send_message;

use {defmt_rtt as _, panic_probe as _};

use embassy_futures::select::{Either, select};
use embassy_stm32::{mode::Async, usart::UartTx};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Receiver};
use embassy_time::{Duration, Instant, Timer};

use crate::state::GLOBAL_STATE;

/// Send telemetry messages over UART
///
/// - Airspeed @ 10 Hz
/// - IMU quaternion @ 20 Hz
/// - Vibration metrics @ 1 Hz
/// - Barometer data @ 1 Hz
/// - LIDAR distance @ 5 Hz
#[embassy_executor::task]
pub async fn serial_uart_tx_task(
    mut tx: UartTx<'static, Async>,
    qei_recv: Receiver<'static, CriticalSectionRawMutex, (u16, bool, bool), 2>,
) {
    let mut counter = 0;
    loop {
        let reason = select(Timer::after(Duration::from_hz(100)), qei_recv.receive()).await;

        use aerosmart_shared::serial::*;
        match reason {
            Either::First(_) => {
                let (
                    airspeed,
                    imu,
                    quat,
                    vibration,
                    baro,
                    lidar,
                    voltage_v,
                    soc_p,
                    pressure_analog_pa,
                    acoustic_data,
                ) = {
                    let state = GLOBAL_STATE.lock().await;
                    let last_idx = (state.imu_head + state.imu_buffer[0].len() - 1)
                        % state.imu_buffer[0].len();

                    let imu_data: [f32; 3] =
                        core::array::from_fn(|i| state.imu_buffer[i][last_idx]);

                    (
                        state.airspeed_meters_per_second,
                        imu_data,
                        state.quaternion,
                        state.vibration_metrics,
                        state.barometer_data,
                        state.lidar_data,
                        state.battery_voltage_volts,
                        state.battery_soc_percent,
                        state.analog_pressure_sensor_data_pa,
                        state.acoustic_data,
                    )
                };
                // Timer elapsed
                counter += 1;

                // 1 Hz task
                if counter % 100 == 0 {
                    if let Some(vibration_metrics) = vibration {
                        // Send vibration metrics @ 1 Hz
                        let vibration_message = SerialMessage::ImuVibrationMetrics {
                            accel_z: vibration_metrics[0].into(),
                            gyro_x: vibration_metrics[1].into(),
                            gyro_y: vibration_metrics[2].into(),
                        };
                        let (buffer, len) = send_message(vibration_message).await;
                        tx.write(&buffer[..len]).await.ok();
                    }
                    if let Some(baro_data) = baro {
                        // Send barometer data @ 1 Hz

                        let baro_message = SerialMessage::BarometerData(baro_data);
                        let (buffer, len) = send_message(baro_message).await;
                        tx.write(&buffer[..len]).await.ok();
                    }
                    // Send battery data @ 1 Hz

                    let battery_message = SerialMessage::BatteryData(BatteryData {
                        voltage_v,
                        soc_percent: soc_p,
                        time_elapsed_ms: Instant::now().as_millis(),
                    });
                    let (buffer, len) = send_message(battery_message).await;
                    tx.write(&buffer[..len]).await.ok();
                }

                // 5 Hz task
                if counter % 20 == 0 {
                    // Send LIDAR data @ 5 Hz
                    if let Some(lidar_data) = lidar {
                        let lidar_message = SerialMessage::LidarData(lidar_data);
                        let (buffer, len) = send_message(lidar_message).await;
                        tx.write(&buffer[..len]).await.ok();
                    }
                }

                // 10 Hz task
                if counter % 10 == 0 {
                    // Send airspeed @ 10 Hz
                    let airspeed_message = SerialMessage::PitotAirspeedData(PitotAirspeedData {
                        splitter_left: 0f32,
                        splitter_right: 0f32,
                        static_port: airspeed,
                        time_elapsed_ms: Instant::now().as_millis(),
                    });
                    let (buffer, len) = send_message(airspeed_message).await;
                    tx.write(&buffer[..len]).await.ok();

                    // Send analog pressure sensor data @ 10 Hz
                    if let Some(pressure_data) = pressure_analog_pa {
                        let analog_pressure_message =
                            SerialMessage::AnalogPressureSensorData(pressure_data);
                        let (buffer, len) = send_message(analog_pressure_message).await;
                        tx.write(&buffer[..len]).await.ok();
                    }
                }

                // 20 Hz task
                if counter % 5 == 0 {
                    if let Some(quat) = quat {
                        // Send IMU quaternion @ 20 Hz
                        let imu_message = SerialMessage::ImuData(ImuData {
                            accel_z: imu[0],
                            gyro_x: imu[1],
                            gyro_y: imu[2],
                            quad_w: quat.w,
                            quad_i: quat.i,
                            quad_j: quat.j,
                            quad_k: quat.k,
                            time_elapsed_ms: Instant::now().as_millis(),
                        });
                        let (buffer, len) = send_message(imu_message).await;
                        tx.write(&buffer[..len]).await.ok();
                    }

                    if let Some(acoustic) = acoustic_data {
                        // Send acoustic data @ 20 Hz
                        let acoustic_message = SerialMessage::AcousticData(acoustic);
                        let (buffer, len) = send_message(acoustic_message).await;
                        tx.write(&buffer[..len]).await.ok();
                    }
                }

                if counter >= 100 {
                    counter = 0;
                }
            }
            Either::Second((qei_cnt, qei_dir, pressed)) => {
                let qei_message = QeiData {
                    position_counts: qei_cnt,
                    direction: qei_dir,
                    pressed,
                    time_elapsed_ms: Instant::now().as_millis(),
                };
                let (buffer, len) = send_message(SerialMessage::QeiData(qei_message)).await;
                tx.write(&buffer[..len]).await.ok();
            }
        }
    }
}
