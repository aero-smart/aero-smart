use libm::expf;

pub mod consts {
    pub const E0_PA: f32 = 611.2;
    pub const MAGNUS_A_DIMLESS: f32 = 17.67;
    pub const MAGNUS_B_C: f32 = 243.5;

    pub const ENHANCEMENT_FACTOR_DIMLESS: f32 = 1.0007 + (3.46e-6 * 101325.0);

    pub const R_D_JOULES_PER_KG_K: f32 = 287.05;
    pub const R_V_JOULES_PER_KG_K: f32 = 461.495;
}

#[inline(always)]
pub fn saturation_vapor_pressure_pa(temperature_c: f32) -> f32 {
    use consts::*;
    E0_PA * expf((MAGNUS_A_DIMLESS * temperature_c) / (temperature_c + MAGNUS_B_C))
}

#[inline(always)]
pub fn vapor_pressure_pa(temperature_c: f32, relative_humidity: f32) -> f32 {
    use consts::*;
    saturation_vapor_pressure_pa(temperature_c)
        * (relative_humidity / 100.0)
        * ENHANCEMENT_FACTOR_DIMLESS
}

#[inline(always)]
pub fn density_kg_per_m3(pressure_pa: f32, temperature_c: f32, relative_humidity: f32) -> f32 {
    use consts::*;
    let t_kelvin = temperature_c + 273.15;
    let p_vapor = vapor_pressure_pa(temperature_c, relative_humidity);
    let p_dry = pressure_pa - p_vapor;

    (p_dry / (R_D_JOULES_PER_KG_K * t_kelvin)) + (p_vapor / (R_V_JOULES_PER_KG_K * t_kelvin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_kg_per_m3() {
        let pressure_pa = 101325.0;
        let temperature_c = 20.0;
        let relative_humidity = 50.0;

        let density = density_kg_per_m3(pressure_pa, temperature_c, relative_humidity);
        assert!((density - 1.184).abs() < 0.01);
    }
}
