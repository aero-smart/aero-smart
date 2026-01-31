use aerosmart_shared::serial::AnalogPressureSensorData;

/// The XGZP6847A uses the voltage among 0.5 V to 4.5 V to represent the pressure from 0 Pa to its maximum range.
/// It's roughly linear within this range.
pub fn voltage_v_to_pressure_pa(
    voltage_v: f32,
    pressure_range_pa_min_left_max_right: (f32, f32),
) -> f32 {
    if !(0.5..=4.5).contains(&voltage_v) {
        defmt::warn!("XGZP6847A voltage out of range: {} V", voltage_v);
    }

    let (pressure_min_pa, pressure_max_pa) = pressure_range_pa_min_left_max_right;

    if voltage_v <= 0.5 {
        return pressure_min_pa;
    } else if voltage_v >= 4.5 {
        return pressure_max_pa;
    }

    ((voltage_v - 0.5) / 4.0) * (pressure_max_pa - pressure_min_pa) + pressure_min_pa
}

pub fn option_arr_to_messsage(input: [Option<f32>; 4]) -> AnalogPressureSensorData {
    let mut output = [0.0; 4];
    let mut bitmask: u8 = 0;

    // Higher bit means lower channel, starting from bit 7 for channel 0
    for (i, opt) in input.iter().enumerate() {
        if let Some(value) = opt {
            output[i] = *value;
            bitmask |= 1 << (7 - i);
        }
    }

    AnalogPressureSensorData {
        pressures_pa: output,
        valid_bitmask: bitmask,
        time_elapsed_ms: embassy_time::Instant::now().as_millis(),
    }
}
