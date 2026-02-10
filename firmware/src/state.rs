use aerosmart_shared::serial::{
    AcousticData, AnalogPressureSensorData, BarometerData, LidarData, SensorConfig,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
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

    pub analog_pressure_sensor_data_pa: Option<AnalogPressureSensorData>, // in Pascals

    pub qei_position_counts: u16,
    pub qei_direction: bool,

    pub acoustic_data: Option<AcousticData>,
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
            battery_voltage_volts: 16.0,
            battery_soc_percent: 100.0,
            analog_pressure_sensor_data_pa: None,
            qei_position_counts: 0,
            qei_direction: true,
            acoustic_data: None,
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}

pub const FFT_SIZE: usize = 4096;
const HALF_DMA_BUFFER_LENGTH: usize = FFT_SIZE * 2; // Stereo: 2 channels
const DMA_BUFFER_LENGTH: usize = HALF_DMA_BUFFER_LENGTH * 2; // Double buffer
pub static mut SAI_BUFFER: [u32; DMA_BUFFER_LENGTH + 512] = [0u32; { DMA_BUFFER_LENGTH + 512 }];

pub type AcousticFftInput = [u32; FFT_SIZE];

pub static GLOBAL_STATE: Mutex<CriticalSectionRawMutex, GlobalState> =
    Mutex::new(GlobalState::new());
pub static AIRSPEED_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static IMU_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static STATUS_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static IMU_BUFFER_FULL_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static DESIRED_UPDATE_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static ANALOG_PRESSURE_SENSOR_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static AUDIO_CHANNEL: Channel<CriticalSectionRawMutex, AcousticFftInput, 2> = Channel::new();
