pub const MAX_AIRSPEED_METERS_PER_SECOND: f32 = 24.0;

pub mod edf {
    use core::f32::consts::PI;
    pub const EDF_DIAMETER_METERS: f32 = 0.0694;
    pub const EDF_AREA_SQ_METERS: f32 =
        PI * (EDF_DIAMETER_METERS / 2.0) * (EDF_DIAMETER_METERS / 2.0);
    pub const EDF_COUNT: usize = 2;

    pub const MAX_THROTTLE_KG: f32 = 1.8;
    pub const MAX_THROTTLE_NEWTONS: f32 = MAX_THROTTLE_KG * 9.81;
    // 2800 KV motor with 4S LiPo (14.8V nominal)
    pub const MAX_RPM: f32 = 2800.0 * 16.8;
    pub const MAX_ANGULAR_VELOCITY_RADIANS_PER_SECOND: f32 = (MAX_RPM / 60.0) * 2.0 * PI;
}

pub mod test_section {
    // pub const TEST_SECTION_WIDTH_METERS: f32 = 0.160;
    // pub const TEST_SECTION_HEIGHT_METERS: f32 = 0.096;
    pub const TEST_SECTION_WIDTH_METERS: f32 = 0.090;
    pub const TEST_SECTION_HEIGHT_METERS: f32 = 0.090;
    pub const TEST_SECTION_LENGTH_METERS: f32 = 0.200;
    pub const TEST_SECTION_CROSS_SECTIONAL_AREA_SQ_METERS: f32 =
        TEST_SECTION_WIDTH_METERS * TEST_SECTION_HEIGHT_METERS;
}
