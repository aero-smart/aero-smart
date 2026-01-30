use crate::{algorithms::motion_fft::compute_motion_fft, state::IMU_BUFFER_FULL_SIGNAL};

use {defmt_rtt as _, panic_probe as _};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn imu_fft_task() {
    loop {
        IMU_BUFFER_FULL_SIGNAL.wait().await;
        let imu_buffer = {
            let state = GLOBAL_STATE.lock().await;
            state.imu_buffer
        };
        let mut results = [None; 3];
        for axis in 0..3 {
            let mut imu_data = imu_buffer[axis];
            let fft_output = compute_motion_fft(&mut imu_data);
            let vibration_metrics =
                crate::algorithms::motion_fft::analyze_vibration(&fft_output, 1000.0);
            defmt::info!(
                "IMU Axis {} Vibration Metrics: RMS Vibration: {} | Dominant Frequency: {} Hz | Peak Magnitude: {}",
                axis,
                vibration_metrics.rms_vibration,
                vibration_metrics.dominant_frequency_hz,
                vibration_metrics.peak_magnitude
            );
            results[axis] = Some(vibration_metrics);
        }
        {
            let mut state = GLOBAL_STATE.lock().await;
            state.vibration_metrics = Some(results.map(|x| x.unwrap()));
        }
    }
}
