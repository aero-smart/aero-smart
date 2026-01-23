pub struct GlobalState {
    pub machine_status: MachineStatus,
    pub control_mode: ControlMode,

    pub imu_buffer: [(i16, i16, i16, i16, i16, i16); 1024],
    pub airspeed_buffer: [u8; 256],
}

pub enum MachineStatus {
    Idle,
    Initializing,
    Running,
    Error,
    EmergencyStop,
}

pub enum ControlMode {
    Manual,
    Assisted,
    Autonomous,
}