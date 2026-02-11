use crate::{
    algorithms::madgwick::MadgwickAhrs,
    sensors::imu_spi::ImuSpi,
    state::{IMU_BUFFER_FULL_SIGNAL, IMU_UPDATED_SIGNAL},
};

use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::exti::ExtiInput;
use embassy_time::{Duration, Timer};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn imu_task(
    mut imu: ImuSpi<'static>,
    mut input: ExtiInput<'static>,
    mut ahrs: MadgwickAhrs,
) {
    loop {
        // Poll @ 1 kHz
        // input.wait_for_falling_edge().await;
        Timer::after(Duration::from_millis(1)).await;
        match imu.poll().await {
            Ok(data) => {
                defmt::info!(
                    "Accel: x={} y={} z={} | Gyro: x={} y={} z={}",
                    data.accel_x,
                    data.accel_y,
                    data.accel_z,
                    data.gyro_x,
                    data.gyro_y,
                    data.gyro_z
                );
                let mut update_ahrs = true;
                if data.accel_x == 0.0 && data.accel_y == 0.0 && data.accel_z == 0.0 {
                    defmt::warn!("IMU returned all-zero data, skipping AHRS update");
                    update_ahrs = false;
                }
                if data.gyro_x == 0.0 && data.gyro_y == 0.0 && data.gyro_z == 0.0 {
                    defmt::warn!("IMU returned all-zero gyro data, skipping AHRS update");
                    update_ahrs = false;
                }
                if update_ahrs {
                    ahrs.update(
                        [data.accel_x, data.accel_y, data.accel_z],
                        [
                            data.gyro_x.to_radians(),
                            data.gyro_y.to_radians(),
                            data.gyro_z.to_radians(),
                        ],
                    );
                }
                defmt::info!(
                    "Quaternion: w={} i={} j={} k={}",
                    ahrs.quaternion.w,
                    ahrs.quaternion.i,
                    ahrs.quaternion.j,
                    ahrs.quaternion.k
                );
                let euler = ahrs.quaternion.euler_angles();
                defmt::info!(
                    "Euler Angles: Roll={}° Pitch={}° Yaw={}°",
                    euler.0.to_degrees(),
                    euler.1.to_degrees(),
                    euler.2.to_degrees()
                );
                {
                    let mut state = GLOBAL_STATE.lock().await;
                    let imu_head = state.imu_head;
                    state.imu_buffer[0][imu_head] = data.accel_z;
                    state.imu_buffer[1][imu_head] = data.gyro_x;
                    state.imu_buffer[2][imu_head] = data.gyro_y;
                    state.imu_head = (state.imu_head + 1) % state.imu_buffer.len();
                    if state.imu_head == 0 {
                        IMU_BUFFER_FULL_SIGNAL.signal(());
                    }
                    IMU_UPDATED_SIGNAL.signal(());
                    if update_ahrs {
                        state.quaternion = Some(ahrs.quaternion);
                    }
                }
            }
            Err(e) => {
                defmt::error!("IMU Error: {:?}", e);
            }
        }
    }
}
