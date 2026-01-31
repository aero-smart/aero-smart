//! Feedforward airspeed calculation
//!
//! The feedforward airspeed is calculated based on the desired airspeed
//! and the characteristics of the propulsion system.
//!
//! ## Theory
//!
//! ### Newton's Second Law
//!
//! $$
//! F_{\mathrm{thrust}} = \dot m \cdot \Delta v = \rho \cdot A \cdot v^2
//! $$
//!
//! where:
//! - $ \dot m $ is the mass flow rate through the propulsion system
//! - $ \Delta v $ is the change in velocity imparted by the propulsion system
//! - $ \rho $ is the air density
//! - $ A $ is the cross-sectional area of the propulsion system
//! - $ v $ is the airspeed through the propulsion system
//!
//! Hence, we can have $v = \sqrt{\dfrac{F_{\mathrm{thrust}}}{\rho \cdot A}}$
//!
//! ### Propulsion System Model
//!
//! Assuming a simple model where the thrust produced by the propulsion system is proportional to the square of the airspeed:
//!
//! $$
//! F_{\mathrm{thrust}} = k_T \cdot \rho \cdot n^2 \cdot D^4
//! $$
//!
//! where:
//! - $ k_T $ is the thrust coefficient
//! - $ \rho $ is the air density
//! - $ n $ is the rotational speed of the propulsion system (in revolutions per second)
//! - $ D $ is the diameter of the propulsion system
//!
//! ### Mass Conservation
//!
//! The mass flow rate through the propulsion system can be related to the airspeed and cross-sectional area:
//!
//! $$
//! \dot m = \rho_1 \cdot A_1 \cdot v_1 = \rho_2 \cdot A_2 \cdot v_2
//! $$
//!
//! where:
//! - $ \rho_1, A_1, v_1 $ are the density, area, and velocity at the Area 1
//! - $ \rho_2, A_2, v_2 $ are the density, area, and velocity at the Area 2
//!
//! Because the airspeed is low (less than 0.3 Mach), we can assume incompressible flow, leading to $ \rho_1 \approx \rho_2 $.
//!
//! Hence, we can simplify the mass flow rate equation to:
//!
//! $$
//! A_{\mathrm{test\ section}}\cdot v_{\mathrm{test\ section}} = A_{\mathrm{edf}}\cdot v_{\mathrm{edf}}
//! $$
//!
//! ### Throttle Linearity
//!
//! Given the specifications of the EDF, we can assume a linear relationship between the throttle command and the RPM (revolutions per minute):
//!
//! $$
//! n \propto \mathrm{throttle}
//! $$
//!
//! ### RPM calculations
//!
//! Given the voltage $V$ supplied to the motor and its KV rating (RPM per volt), we can calculate the RPM as:
//!
//! $$
//! \mathrm{RPM} = \mathrm{KV} \cdot V \cdot \mathrm{throttle\ (percentage)}
//! $$
//!
//! Hence, we can assume it **linear** between the desired airspeed and the throttle command.

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
    sqrtf(edf::MAX_THROTTLE_KG / (air_density_kg_per_cubic_meter * edf::EDF_AREA_SQ_METERS))
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
    battery_voltage_cell_avg: f32,
) -> u16 {
    use num_traits::Float;
    let v_edf_max_standard_meters_per_second =
        max_edf_airspeed_meters_per_second(air_density_kg_per_cubic_meter);

    let throttle_normalized =
        desired_airspeed_meters_per_second / v_edf_max_standard_meters_per_second;

    // This compensates for voltage sag in the battery. Higher voltage allows for higher RPM and thus higher airspeed.
    let voltage_compensated = throttle_normalized / (battery_voltage_cell_avg / 4.2f32);

    // Clamp to [0.0, 1.0]
    let throttle = voltage_compensated.clamp(0.0, 1.0);

    let dshot_throttle_value = (throttle * 2000.0).round() as u16;

    dshot_throttle_value.min(2000)
}
