use libm::sqrtf;

#[inline]
pub fn calculate_airspeed(pressure_diff_pa: f32, air_density_kg_per_m3: f32) -> f32 {
    if pressure_diff_pa <= 0.0 {
        0.0
    } else {
        sqrtf(2.0 * pressure_diff_pa / air_density_kg_per_m3)
    }
}
