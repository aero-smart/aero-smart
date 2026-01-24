#[cfg(feature = "sql")]
use pgvector::Vector;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
#[cfg(feature = "serde")]
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct ThrottleConfig {
    pub airspeed: u8,
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct ServoConfig {
    pub angle: u8,
}

#[derive(Debug, Clone, Copy, Default, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct SensorConfig {
    pub imu_horizontal: bool,
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub enum Command {
    Start,
    Stop,
    Calibrate,
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct PitotAirspeedData {
    pub splitter_left: f32,
    pub splitter_right: f32,
    pub static_port: f32,
}

#[cfg(feature = "sql")]
impl PitotAirspeedData {
    pub fn to_record(&self, session: Uuid) -> crate::sql::PitotAirspeedRecord {
        crate::sql::PitotAirspeedRecord {
            id: 0, // ID will be set by the database
            session,
            timestamp: chrono::Utc::now(),
            splitter_left: self.splitter_left,
            splitter_right: self.splitter_right,
            static_port: self.static_port,
        }
    }
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct ImuData {
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub quad_w: f32,
    pub quad_i: f32,
    pub quad_j: f32,
    pub quad_k: f32,
}

impl ImuData {
    pub fn new(
        accel_z: f32,
        gyro_x: f32,
        gyro_y: f32,
        quad_w: f32,
        quad_i: f32,
        quad_j: f32,
        quad_k: f32,
    ) -> Self {
        Self {
            accel_z,
            gyro_x,
            gyro_y,
            quad_w,
            quad_i,
            quad_j,
            quad_k,
        }
    }
}

#[cfg(feature = "sql")]
impl ImuData {
    pub fn to_record(&self, session: Uuid) -> crate::sql::ImuRecord {
        crate::sql::ImuRecord {
            id: 0, // ID will be set by the database
            session,
            timestamp: chrono::Utc::now(),
            accel_z: self.accel_z,
            gyro_x: self.gyro_x,
            gyro_y: self.gyro_y,
            quad_w: self.quad_w,
            quad_i: self.quad_i,
            quad_j: self.quad_j,
            quad_k: self.quad_k,
        }
    }
}


#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct ImuVibrationMetrics {
    pub rms_vibration: f32,
    pub dominant_frequency_hz: f32,
    pub peak_magnitude: f32,
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct AcousticData {
    /// Overall Sound Pressure Level in dB
    pub overall_spl: f32,
    /// The "white" frequency of the EDF
    pub peak_frequency: f32,

    /// The `Spectral Shape` (4 bins for the first implementation)
    /// Map these to 1/3 octave bands or custom interest zones
    pub spectral_shape: [f32; 4],

    /// Ratio of broadband noise to tonal noise for AI optimization
    pub turbulence_index: f32,
}

#[cfg(feature = "sql")]
impl AcousticData {
    pub fn to_record(&self, session: Uuid) -> crate::sql::AcousticRecord {
        crate::sql::AcousticRecord {
            id: 0, // ID will be set by the database
            session,
            timestamp: chrono::Utc::now(),
            overall_spl: self.overall_spl,
            peak_frequency: self.peak_frequency,
            spectral_shape: Vector::from(self.spectral_shape.to_vec()),
            turbulence_index: self.turbulence_index,
        }
    }
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct LidarData {
    pub distance_cm: u16,
    pub signal_strength: u16,
}

#[derive(Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub struct BarometerData {
    pub pressure_pa: f32,
    pub temperature_c: f32,
    pub humidity_percent: f32,
}

#[derive(Debug, Clone, Archive, RkyvSerialize, RkyvDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[rkyv(derive(Debug))]
pub enum SerialMessage {
    ThrottleConfig(ThrottleConfig),
    ServoConfig(ServoConfig),
    SensorConfig(SensorConfig),
    Command(Command),
    PitotAirspeedData(PitotAirspeedData),
    ImuData(ImuData),
    AcousticData(AcousticData),
    LidarData(LidarData),
    BarometerData(BarometerData),
    ImuVibrationMetrics {
        accel_z: ImuVibrationMetrics,
        gyro_x: ImuVibrationMetrics,
        gyro_y: ImuVibrationMetrics,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rkyv::rancor::Error;

    #[test]
    fn test_throttle_config_serialization() {
        let config = ThrottleConfig {
            airspeed: 10,
        };
        let serialized = rkyv::to_bytes::<Error>(&config).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert_eq!(serialized.len(), 2); // u8 x 2
    }

    #[test]
    fn test_imu_data_serialization() {
        let imu_data = ImuData {
            accel_z: 3.0,
            gyro_x: 4.0,
            gyro_y: 5.0,
            quad_i: 0.1,
            quad_j: 0.2,
            quad_k: 0.3,
            quad_w: 0.4,
        };
        let serialized = rkyv::to_bytes::<Error>(&imu_data).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert_eq!(serialized.len(), 28); // f32 x 7
    }

    #[test]
    fn test_serial_message_serialization() {
        let message = SerialMessage::ServoConfig(ServoConfig { angle: 45 });
        let serialized = rkyv::to_bytes::<Error>(&message).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert!(serialized.len() > 1); // At least 1 byte for the enum tag
    }

    #[test]
    fn test_pitot_airspeed_data_serialization() {
        let data = PitotAirspeedData {
            splitter_left: 12.5,
            splitter_right: 13.5,
            static_port: 14.5,
        };
        let serialized = rkyv::to_bytes::<Error>(&data).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert_eq!(serialized.len(), 12); // f32 x 3
    }

    #[test]
    fn test_acoustic_data_deserialization() {
        let data = AcousticData {
            overall_spl: 85.0,
            peak_frequency: 1500.0,
            spectral_shape: [0.0; 4],
            turbulence_index: 0.5,
        };
        let serialized = rkyv::to_bytes::<Error>(&data).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert_eq!(serialized.len(), 4 + 4 + (4 * 32) + 4); // f32 x (1 + 1 + 32 + 1)
        let to_be_deserialized = serialized.into_vec();
        let archived = rkyv::access::<ArchivedAcousticData, Error>(&to_be_deserialized).unwrap();
        println!("{:?}", archived);
        let deserialized = rkyv::deserialize::<AcousticData, Error>(archived).unwrap();
        assert_eq!(deserialized.overall_spl, data.overall_spl);
    }
}
