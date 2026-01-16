#![cfg(feature = "sql")]

use pgvector::Vector;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct ImuRecord {
    pub id: i32,
    pub session: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PitotAirspeedRecord {
    pub id: i32,
    pub session: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub splitter_left: f32,
    pub splitter_right: f32,
    pub static_port: f32,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AcousticRecord {
    pub id: i32,
    pub session: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub overall_spl: f32,
    pub peak_frequency: f32,
    pub spectral_shape: Vector,
    pub turbulence_index: f32,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Experiment {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}
