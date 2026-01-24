use ahrs::{Ahrs, Madgwick};
use nalgebra::{UnitQuaternion, Vector3};

pub struct MadgwickAhrs {
    pub ahrs: Madgwick<f32>,
    pub quaternion: UnitQuaternion<f32>,
}

#[derive(Debug, defmt::Format)]
pub enum MadgwickAhrsError {
    AccelerometerNormZero,
    MagnetometerNormZero,
}

impl From<ahrs::AhrsError> for MadgwickAhrsError {
    fn from(error: ahrs::AhrsError) -> Self {
        match error {
            ahrs::AhrsError::AccelerometerNormZero => MadgwickAhrsError::AccelerometerNormZero,
            ahrs::AhrsError::MagnetometerNormZero => MadgwickAhrsError::MagnetometerNormZero,
        }
    }
}

impl MadgwickAhrs {
    pub fn new(sample_freq_hz: f32, beta: f32) -> Self {
        Self {
            ahrs: Madgwick::new(sample_freq_hz, beta),
            quaternion: UnitQuaternion::identity(),
        }
    }

    pub fn update(&mut self, gyro_rad_per_sec: [f32; 3], accel_meters_per_sec_squared: [f32; 3]) {
        let gyro_vector = Vector3::from_column_slice(&gyro_rad_per_sec);
        let accel_vector = Vector3::from_column_slice(&accel_meters_per_sec_squared);
        match self.ahrs.update_imu(&gyro_vector, &accel_vector) {
            Ok(quat) => {
                self.quaternion = *quat;
            }
            Err(e) => {
                let err = MadgwickAhrsError::from(e);
                defmt::warn!("Madgwick AHRS update failed: {:?}", err);
            }
        }
    }

    pub fn euler_angles(&self) -> Vector3<f32> {
        let ea = self.quaternion.euler_angles();
        Vector3::new(ea.0, ea.1, ea.2)
    }
}
