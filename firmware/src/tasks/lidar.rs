use crate::{
    sensors::lidar_uart::LidarUart,
    state::{MachineStatus, STATUS_UPDATED_SIGNAL},
};

use {defmt_rtt as _, panic_probe as _};

use embassy_time::{Duration, Timer};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn lidar_task(mut lidar: LidarUart<'static>) {
    loop {
        match lidar.poll().await {
            Ok(distance) => {
                {
                    let mut state = GLOBAL_STATE.lock().await;
                    state.lidar_data = Some(distance);
                }
                defmt::info!("LIDAR Distance: {} mm", distance);
                if distance.signal_strength < 100 {
                    defmt::warn!("LIDAR signal strength low: {}", distance.signal_strength);
                }
                if distance.distance_cm <= 30 {
                    defmt::warn!("LIDAR distance too close: {} cm", distance.distance_cm);
                    {
                        let mut state = GLOBAL_STATE.lock().await;
                        state.machine_status = MachineStatus::EmergencyStop;
                        STATUS_UPDATED_SIGNAL.signal(());
                    }
                }
            }
            Err(e) => {
                defmt::error!("LIDAR Error: {:?}", e);
            }
        }
        Timer::after(Duration::from_secs(3)).await;
    }
}
