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

pub mod sensors {
    pub const IMU_SAMPLE_RATE_HZ: u32 = 1_000;
    pub const IMU_FFT_SAMPLE_COUNT: usize = 4_096;
    pub const IMU_FFT_WINDOW_SIZE: usize = 256;
    pub const IMU_FFT_OVERLAP: usize = 128;

    pub const AIRSPEED_SAMPLE_RATE_HZ: u32 = 20;
    pub const BAROMETER_SAMPLE_RATE_HZ: u32 = 1;

    pub const LIDAR_SAMPLE_RATE_HZ: u32 = 10;

    pub const UPPER_BAUD_RATE: u32 = 915_200;
    pub const LIDAR_BAUD_RATE: u32 = 115_200;

    pub const I2C_FREQ_KHZ: u32 = 400;

    pub const SERVO_PWM_FREQUENCY_HZ: u32 = 50;
    pub const EDF_PWM_FREQUENCY_HZ: u32 = 50;

    pub const MICROPHONE_SAMPLE_RATE_HZ: u32 = 44_100;

    pub const ANALOG_PRESSURE_SENSOR_SAMPLE_RATE_HZ: u32 = 5;

    pub const BATTERY_VOLTAGE_SAMPLE_RATE_HZ: u32 = 1;

    pub const LIDAR_SAMPLE_RATE_SECS: u32 = 3;
}

pub mod algorithms {
    pub const MADGWICK_BETA: f32 = 0.1; // Adjust based on expected motion dynamics

    pub const PID_KP: f32 = 0.24;
    pub const PID_KI: f32 = 0.08;
    pub const PID_KD: f32 = 0.06;

    pub const PID_OUTPUT_MIN: f32 = 0.0;
    pub const PID_OUTPUT_MAX: f32 = 1.0;
}
