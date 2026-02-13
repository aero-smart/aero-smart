use crate::{
    algorithms::airspeed::AirspeedControl,
    executors::edf_pwm::EdfPwm,
    state::{AIRSPEED_UPDATED_SIGNAL, MachineStatus},
};

use {defmt_rtt as _, panic_probe as _};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn edf_task(mut edf: EdfPwm, mut pid: AirspeedControl) {
    edf.initialize().await;
    loop {
        AIRSPEED_UPDATED_SIGNAL.wait().await;
        let (measured, setpoint, status, density, voltage_v) = {
            let state = GLOBAL_STATE.lock().await;
            (
                state.airspeed_meters_per_second,
                state.desired_airspeed_meters_per_second,
                state.machine_status,
                state.air_density_kg_per_cubic_meter,
                state.battery_voltage_volts,
            )
        };

        if setpoint == 0.0 {
            edf.set_throttle_compatible(0);
            continue;
        }

        match status {
            MachineStatus::Running => {
                defmt::info!(
                    "EDF Control | Measured Airspeed: {} m/s | Setpoint: {} m/s",
                    measured,
                    setpoint
                );
                pid.update_setpoint(setpoint);
                let edf_airspeed = pid.compute_throttle(measured, density, voltage_v);
                edf.set_throttle_compatible(edf_airspeed);
            }
            MachineStatus::EmergencyStop => {
                defmt::warn!("Emergency Stop Engaged! Cutting off EDF throttle.");
                edf.stop();
            }
            _ => {}
        }
    }
}
