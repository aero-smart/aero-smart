use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;

#[derive(Clone, Copy, Debug, PartialEq, defmt::Format)]
pub struct GlobalState {
    pub machine_status: MachineStatus,
    pub control_mode: ControlMode,

    pub air_density_kg_per_cubic_meter: f32,
    pub airspeed_meters_per_second: f32,
    pub desired_airspeed_meters_per_second: f32,

    pub imu_buffer: [(i16, i16, i16, i16, i16, i16); 1024],
    pub airspeed_buffer: [u8; 256],

    pub imu_head: usize,
    pub airspeed_head: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, defmt::Format)]
pub enum MachineStatus {
    Idle,
    Initializing,
    Running,
    Error,
    EmergencyStop,
}

#[derive(Clone, Copy, Debug, PartialEq, defmt::Format)]
pub enum ControlMode {
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
            imu_buffer: [(0, 0, 0, 0, 0, 0); 1024],
            airspeed_buffer: [0; 256],
            imu_head: 0,
            airspeed_head: 0,
        }
    }
}

pub static GLOBAL_STATE: Mutex<CriticalSectionRawMutex, GlobalState> =
    Mutex::new(GlobalState::new());
pub static AIRSPEED_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static IMU_UPDATED_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
