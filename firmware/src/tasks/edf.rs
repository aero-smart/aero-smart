use crate::{
    algorithms::airspeed::AirspeedControl,
    executors::edf::EdfDshot,
    state::{AIRSPEED_UPDATED_SIGNAL, MachineStatus, STATUS_UPDATED_SIGNAL},
};

use {defmt_rtt as _, panic_probe as _};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn edf_task(mut edf: EdfDshot<'static>, mut pid: AirspeedControl) {
    loop {
        AIRSPEED_UPDATED_SIGNAL.wait().await;
        let (measured, setpoint, status, density) = {
            let state = GLOBAL_STATE.lock().await;
            (
                state.airspeed_meters_per_second,
                state.desired_airspeed_meters_per_second,
                state.machine_status,
                state.air_density_kg_per_cubic_meter,
            )
        };

        match status {
            MachineStatus::Running => {
                defmt::info!(
                    "EDF Control | Measured Airspeed: {} m/s | Setpoint: {} m/s",
                    measured,
                    setpoint
                );
                pid.update_setpoint(setpoint);
                let edf_airspeed = pid.compute_throttle(measured, density);
                match edf.set_throttle_symmetric(edf_airspeed).await {
                    Ok(_) => {}
                    Err(e) => {
                        defmt::error!("EDF Control Error: {:?}", e);
                        {
                            let mut state = GLOBAL_STATE.lock().await;
                            state.machine_status = MachineStatus::Error;
                            STATUS_UPDATED_SIGNAL.signal(());
                        }
                    }
                }
            }
            MachineStatus::EmergencyStop => {
                defmt::warn!("Emergency Stop Engaged! Cutting off EDF throttle.");
                match edf.stop().await {
                    Ok(_) => {}
                    Err(e) => {
                        defmt::error!("EDF Control Error during Emergency Stop: {:?}", e);
                    }
                }
            }
            _ => {}
        }
    }
}
