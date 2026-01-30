use crate::utils::send_message;

use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::{mode::Async, usart::UartTx};
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
pub async fn serial_uart_tx_task(mut tx: UartTx<'static, Async>) {
    let mut counter = 0;
    loop {
        Timer::after(Duration::from_millis(100)).await;
        counter += 1;
        use aerosmart_shared::serial::*;
        let (airspeed, imu, quat, vibration, baro, lidar) = {
            let state = GLOBAL_STATE.lock().await;
            let last_idx =
                (state.imu_head + state.imu_buffer[0].len() - 1) % state.imu_buffer[0].len();

            let imu_data: [f32; 3] = core::array::from_fn(|i| state.imu_buffer[i][last_idx]);

            (
                state.airspeed_meters_per_second,
                imu_data,
                state.quaternion,
                state.vibration_metrics,
                state.barometer_data,
                state.lidar_data,
            )
        };
        // Send airspeed @ 10 Hz
        let airspeed_message = SerialMessage::PitotAirspeedData(PitotAirspeedData {
            splitter_left: 0f32,
            splitter_right: 0f32,
            static_port: airspeed,
            time_elapsed_ms: Instant::now().as_millis(),
        });
        let (buffer, len) = send_message(airspeed_message).await;
        tx.write(&buffer[..len]).await.ok();

        // Send IMU quaternion @ 20 Hz

        if counter % 2 == 0
            && let Some(quat) = quat
        {
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

        // Send vibration metrics @ 1 Hz
        if counter % 10 == 0
            && let Some(vibration_metrics) = vibration
        {
            let vibration_message = SerialMessage::ImuVibrationMetrics {
                accel_z: vibration_metrics[0].into(),
                gyro_x: vibration_metrics[1].into(),
                gyro_y: vibration_metrics[2].into(),
            };
            let (buffer, len) = send_message(vibration_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        if counter % 10 == 0
            && let Some(baro_data) = baro
        {
            // Send barometer data @ 1 Hz

            let baro_message = SerialMessage::BarometerData(baro_data);
            let (buffer, len) = send_message(baro_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        if counter % 2 == 0
            && let Some(lidar_data) = lidar
        {
            // Send LIDAR data @ 5 Hz

            let lidar_message: SerialMessage = SerialMessage::LidarData(lidar_data);
            let (buffer, len) = send_message(lidar_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        if counter >= 100 {
            counter = 0;
        }
    }
}
