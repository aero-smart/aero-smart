/// Given the airspeed less than 0.3 Mach, we can ignore the compressibility effects of air in its density calculation.
/// Thus, we can use the incompressible form of the mass conservation equation to estimate airspeed through the test section.
/// This function computes the airspeed in meters per second given the volumetric flow rate in cubic meters per second
/// and the cross-sectional area of the test section in square meters.
#[inline(always)]
pub fn edf_airspeed_meters_per_second(test_section_meters_per_second: f32) -> f32 {
    use crate::consts::*;
    // S_edf * v_edf = S_test_section * v_test_section

    (test_section::TEST_SECTION_CROSS_SECTIONAL_AREA_SQ_METERS * test_section_meters_per_second)
        / (edf::EDF_AREA_SQ_METERS * edf::EDF_COUNT as f32)
}

pub fn max_edf_airspeed_meters_per_second(air_density_kg_per_cubic_meter: f32) -> f32 {
    // F = rho * A * v^2
    // v = sqrt(F / (rho * A))
    use crate::consts::edf;
    use libm::sqrtf;

    // Since it's one EDF generating the thrust, we don't need to multiply by EDF_COUNT here.
    sqrtf(edf::MAX_THROTTLE_KG / (air_density_kg_per_cubic_meter * edf::EDF_AREA_SQ_METERS as f32))
}

/// Outputs Dshot throttle value (0–2000) normalized for the desired airspeed in meters per second.
/// The `dshot-frame` crate expects throttle values in the range of 0–2000 for Dshot commands and in its internal logic adds the offset `47`.
///
/// Linearity proof:
/// - At 0 m/s desired airspeed, throttle = 0 / v_edf_max = 0 -> Dshot value = 0
/// - At v_edf_max desired airspeed, throttle = v_edf_max / v_edf_max = 1 -> Dshot value = 2000
pub fn edf_throttle_from_airspeed(
    desired_airspeed_meters_per_second: f32,
    air_density_kg_per_cubic_meter: f32,
) -> u16 {
    use num_traits::Float;
    let v_edf_max_standard_meters_per_second =
        max_edf_airspeed_meters_per_second(air_density_kg_per_cubic_meter);

    let throttle_normalized =
        desired_airspeed_meters_per_second / v_edf_max_standard_meters_per_second;

    // Clamp to [0.0, 1.0]
    let throttle = throttle_normalized.clamp(0.0, 1.0);

    let dshot_throttle_value = (throttle * 2000.0).round() as u16;

    dshot_throttle_value.min(2000)
}
