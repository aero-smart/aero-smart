use pgvector::Vector;
use serde::{Deserialize, Serialize};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(derive(Debug))]
pub struct ThrottleConfig {
    pub left: u8,
    pub right: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(derive(Debug))]
pub struct ServoConfig {
    pub angle: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(derive(Debug))]
pub struct PitotAirspeedData {
    pub splitter_left: f32,
    pub splitter_right: f32,
    pub static_port: f32,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(derive(Debug))]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

impl ImuData {
    pub fn to_record(&self, session: Uuid) -> crate::sql::ImuRecord {
        crate::sql::ImuRecord {
            id: 0, // ID will be set by the database
            session,
            timestamp: chrono::Utc::now(),
            accel_x: self.accel_x,
            accel_y: self.accel_y,
            accel_z: self.accel_z,
            gyro_x: self.gyro_x,
            gyro_y: self.gyro_y,
            gyro_z: self.gyro_z,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(derive(Debug))]
pub struct AcousticData {
    /// Overall Sound Pressure Level in dB
    pub overall_spl: f32,
    /// The "white" frequency of the EDF
    pub peak_frequency: f32,

    /// The `Spectral Shape` (32 bins)
    /// Map these to 1/3 octave bands or custom interest zones
    pub spectral_shape: [f32; 32],

    /// Ratio of broadband noise to tonal noise for AI optimization 
    pub turbulence_index: f32,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(derive(Debug))]
pub enum SerialMessage {
    ThrottleConfig(ThrottleConfig),
    ServoConfig(ServoConfig),
    PitotAirspeedData(PitotAirspeedData),
    ImuData(ImuData),
    AcousticData(AcousticData),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rkyv::rancor::Error;
    
    #[test]
    fn test_throttle_config_serialization() {
        let config = ThrottleConfig { left: 100, right: 150 };
        let serialized = rkyv::to_bytes::<Error>(&config).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert_eq!(serialized.len(), 2); // u8 x 2
    }

    #[test]
    fn test_imu_data_serialization() {
        let imu_data = ImuData {
            accel_x: 1.0,
            accel_y: 2.0,
            accel_z: 3.0,
            gyro_x: 4.0,
            gyro_y: 5.0,
            gyro_z: 6.0,
        };
        let serialized = rkyv::to_bytes::<Error>(&imu_data).unwrap();
        assert_eq!(serialized.len() > 0, true);
        assert_eq!(serialized.len(), 24); // f32 x 6
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
            spectral_shape: [0.0; 32],
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
