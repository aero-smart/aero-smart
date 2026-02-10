use crate::{
    sensors::pitot_i2c::Airspeed,
    state::AIRSPEED_UPDATED_SIGNAL,
    utils::{magnus::density_kg_per_m3, pitot::calculate_airspeed},
};

use {defmt_rtt as _, panic_probe as _};

use embassy_time::{Duration, Timer};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
/// Poll pitot tube @ 100 Hz and barometer @ 10 Hz
pub async fn airspeed_task(mut sensors: Airspeed) {
    let mut counter = 0;
    loop {
        match sensors.read_pitot().await {
            Ok((status, pressure_raw, temperature_raw)) => {
                if counter % 10 == 0 {
                    defmt::info!(
                        "Pitot Readings | Status: {} | Pressure Raw: {} | Temperature Raw: {}",
                        status,
                        pressure_raw,
                        temperature_raw
                    );
                }
                {
                    let mut state = GLOBAL_STATE.lock().await;
                    let airspeed =
                        calculate_airspeed(pressure_raw, state.air_density_kg_per_cubic_meter);
                    state.airspeed_meters_per_second = airspeed;
                    if counter % 5 == 0 {
                        defmt::info!("Calculated Airspeed: {} m/s", airspeed);
                    }
                }
                AIRSPEED_UPDATED_SIGNAL.signal(());
            }
            Err(e) => {
                defmt::error!("Airspeed Sensor Error: {:?}", e);
            }
        }

        counter += 1;

        if counter >= 8 {
            match sensors.read_barometer().await {
                Ok(baro_data) => {
                    defmt::info!(
                        "Barometer Pressure: {} Pa | Temperature: {} °C | Humidity: {} %",
                        baro_data.pressure_pa,
                        baro_data.temperature_c,
                        baro_data.humidity_percent
                    );
                    {
                        let mut state = GLOBAL_STATE.lock().await;
                        let density = density_kg_per_m3(
                            baro_data.pressure_pa,
                            baro_data.temperature_c,
                            baro_data.humidity_percent,
                        );
                        state.barometer_data = Some(baro_data);
                        state.air_density_kg_per_cubic_meter = density;
                        defmt::info!("Updated Air Density: {} kg/m^3", density);
                    }
                }
                Err(e) => {
                    defmt::error!("Barometer Error: {:?}", e);
                }
            }
            counter = 0;
        }

        Timer::after(Duration::from_hz(40)).await;
    }
}
