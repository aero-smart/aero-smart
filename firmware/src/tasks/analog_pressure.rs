use embassy_time::{Duration, Timer};

use crate::{consts::sensors::ANALOG_PRESSURE_SENSOR_SAMPLE_RATE_HZ, sensors::adc_i2c::AdcI2c};

#[embassy_executor::task]
pub async fn analog_pressure_task(mut adc: AdcI2c<'static>) {
    loop {
        Timer::after(Duration::from_hz(
            ANALOG_PRESSURE_SENSOR_SAMPLE_RATE_HZ as u64,
        ))
        .await;
        let result = adc.poll_all().await;
        match result {
            Ok(pressures) => {
                for (i, pressure_pa) in pressures
                    .pressures_pa
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (pressures.valid_bitmask & (1 << (7 - i))) != 0)
                {
                    defmt::info!("Analog Pressure Sensor Channel {}: {} Pa", i, pressure_pa);
                }
                {
                    let mut state = crate::state::GLOBAL_STATE.lock().await;
                    state.analog_pressure_sensor_data_pa = Some(pressures);
                }
            }
            Err(e) => {
                defmt::error!("Analog Pressure Sensor Error: {:?}", e);
            }
        }
    }
}
