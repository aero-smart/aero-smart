use pid::Pid;

use crate::consts::MAX_AIRSPEED_METERS_PER_SECOND;

pub struct AirspeedControl {
    pub pid: Pid<f32>,
}

impl AirspeedControl {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            pid: Pid::new(0.0, MAX_AIRSPEED_METERS_PER_SECOND).kp(kp).ki(ki).kd(kd),
        }
    }

    pub fn update_airspeed(&mut self, current_airspeed: f32) -> f32 {
        self.pid.next_control_output(current_airspeed).output
    }

    pub fn update_setpoint(&mut self, new_setpoint: f32) {
        self.pid.setpoint = new_setpoint;
    }
}