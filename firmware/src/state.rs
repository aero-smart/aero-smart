use aerosmart_shared::serial::{BarometerData, LidarData, SensorConfig};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalState {
    pub machine_status: MachineStatus,
    pub control_mode: ControlMode,

    pub air_density_kg_per_cubic_meter: f32,
    pub airspeed_meters_per_second: f32,
    pub desired_airspeed_meters_per_second: f32,
    pub desired_servo_angle_deg: f32,

    pub config: SensorConfig,

    /// IMU buffer for FFT: [accel_z, gyro_x, gyro_y]
    pub imu_buffer: [[f32; 1024]; 3],
    pub airspeed_buffer: [u16; 256],

    pub barometer_data: Option<BarometerData>, // pressure_pa, temperature_c, humidity_percent
    pub lidar_data: Option<LidarData>,
    pub vibration_metrics: Option<[crate::algorithms::motion_fft::VibrationMetrics; 3]>,
    pub quaternion: Option<nalgebra::UnitQuaternion<f32>>,

    pub imu_head: usize,
    pub airspeed_head: usize,

    pub battery_voltage_volts: f32,
    pub battery_soc_percent: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, defmt::Format)]
pub enum MachineStatus {
    #[default]
    Idle,
    Initializing,
    Running,
    Error,
    EmergencyStop,
}

impl MachineStatus {
    pub fn display_led(&self) -> (u8, u8, u8) {
        match self {
            MachineStatus::Idle => (0, 0, 255),            // Blue
            MachineStatus::Initializing => (255, 255, 0),  // Yellow
            MachineStatus::Running => (0, 255, 0),         // Green
            MachineStatus::Error => (255, 0, 0),           // Red
            MachineStatus::EmergencyStop => (255, 0, 255), // Magenta
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, defmt::Format)]
pub enum ControlMode {
    #[default]
    Manual,
    Assisted,
    Autonomous,
}

impl GlobalState {
    pub const fn new() -> Self {
        Self {
            machine_status: MachineStatus::Idle,
            control_mode: ControlMode::Manual,
            air_density_kg_per_cubic_meter: 1.225,
            airspeed_meters_per_second: 0.0,
            desired_airspeed_meters_per_second: 0.0,
            desired_servo_angle_deg: 0.0,
            config: SensorConfig {
                imu_horizontal: false,
            },
            imu_buffer: [[0.0; 1024]; 3],
            airspeed_buffer: [0; 256],
            barometer_data: None,
            lidar_data: None,
            vibration_metrics: None,
            quaternion: None,
            imu_head: 0,
            airspeed_head: 0,
            battery_voltage_volts: 0.0,
            battery_soc_percent: 0.0,
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}

pub static GLOBAL_STATE: Mutex<CriticalSectionRawMutex, GlobalState> =
    Mutex::new(GlobalState::new());
pub static AIRSPEED_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static IMU_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static STATUS_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static IMU_BUFFER_FULL_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static DESIRED_UPDATE_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
