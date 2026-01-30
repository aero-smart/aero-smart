const VOLTAGES: [f32; 21] = [
    3.27, 3.61, 3.69, 3.71, 3.73, 3.75, 3.77, 3.79, 3.80, 3.82, 3.84, 3.85, 3.87, 3.91, 3.95, 3.98,
    4.02, 4.08, 4.11, 4.15, 4.20,
];

const SOC_VALUES: [f32; 21] = [
    0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0,
    80.0, 85.0, 90.0, 95.0, 100.0,
];

fn get_soc(measured_voltage: f32) -> f32 {
    use num_traits::clamp;
    let threshold_min = VOLTAGES[0];
    let threshold_max = VOLTAGES[VOLTAGES.len() - 1];
    let measured_voltage = clamp(measured_voltage, threshold_min, threshold_max);

    let grids: &[&[f32]] = &[&VOLTAGES];
    let obs_v: &[&[f32]] = &[&[measured_voltage]];
    let mut out = [0.0f32; 1];

    let _ = interpn::multilinear::rectilinear::interpn(grids, &SOC_VALUES, obs_v, &mut out);

    out[0]
}

/// Convert ADC reading to battery voltage in volts
///
/// # Arguments
/// * `adc_reading` - Raw ADC value from battery voltage divider
/// * `vrefint_reading` - Raw ADC value from internal reference
/// * `vref_nominal` - Nominal VDDA voltage (typically 3.3V)
///
/// # Returns
/// Total battery pack voltage in volts
fn adc_to_voltage(adc_reading: u16, vrefint_reading: u16, vref_nominal: f32) -> f32 {
    // VREFINT from datasheet: -40°C < TJ < 105°C, VDD = 3.3V
    // Typical: 1.216V, Min: 1.180V, Max: 1.255V
    const VREFINT_CAL: f32 = 1.216; // Typical value for STM32
    const ADC_MAX: f32 = 4095.0; // 12-bit ADC

    // Voltage divider ratio: R1=5k, R2=1k
    // Vmeasured = Vbattery * R2/(R1+R2) = Vbattery * 1/6
    const VOLTAGE_DIVIDER_RATIO: f32 = 6.0; // (5k + 1k) / 1k

    // Calculate actual VDDA from internal reference
    // VDDA = VREFINT_CAL * ADC_MAX / vrefint_reading
    let vdda = VREFINT_CAL * ADC_MAX / (vrefint_reading as f32);

    // Calculate voltage at ADC pin
    let battery_adc_voltage = (adc_reading as f32 / ADC_MAX) * vdda;

    // Calculate actual battery voltage (compensate for voltage divider)
    let battery_voltage = battery_adc_voltage * VOLTAGE_DIVIDER_RATIO;

    battery_voltage
}

/// Convert ADC reading to battery cell voltage in volts
///
/// # Arguments
/// * `adc_reading` - Raw ADC value from battery voltage divider
/// * `vrefint_reading` - Raw ADC value from internal reference
/// * `vref_nominal` - Nominal VDDA voltage (typically 3.3V)
/// * `num_cells` - Number of cells in series (e.g., 4 for 4S LiPo)
///
/// # Returns
/// Per-cell voltage in volts
fn adc_to_cell_voltage(
    adc_reading: u16,
    vrefint_reading: u16,
    vref_nominal: f32,
    num_cells: f32,
) -> f32 {
    let battery_voltage = adc_to_voltage(adc_reading, vrefint_reading, vref_nominal);
    battery_voltage / num_cells
}

/// Convert ADC reading to State of Charge (SOC) percentage
///
/// # Arguments
/// * `adc_reading` - Raw ADC value from battery voltage divider
/// * `vrefint_reading` - Raw ADC value from internal reference
/// * `vref_nominal` - Nominal VDDA voltage (typically 3.3V)
///
/// # Returns
/// Battery SOC percentage (0.0 - 100.0)
pub fn measured_to_soc(adc_reading: u16, vrefint_reading: u16, vref_nominal: f32) -> f32 {
    const NUM_CELLS: f32 = 4.0; // 4S LiPo

    let cell_voltage = adc_to_cell_voltage(adc_reading, vrefint_reading, vref_nominal, NUM_CELLS);
    get_soc(cell_voltage)
}

/// Get battery status information
///
/// # Returns
/// (battery_voltage, cell_voltage, soc_percentage)
pub fn get_battery_info(
    adc_reading: u16,
    vrefint_reading: u16,
    vref_nominal: f32,
) -> (f32, f32, f32) {
    const NUM_CELLS: f32 = 4.0;

    let battery_voltage = adc_to_voltage(adc_reading, vrefint_reading, vref_nominal);
    let cell_voltage = battery_voltage / NUM_CELLS;
    let soc = get_soc(cell_voltage);

    (battery_voltage, cell_voltage, soc)
}
