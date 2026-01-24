use pid::Pid;

use crate::{
    consts::MAX_AIRSPEED_METERS_PER_SECOND, utils::mass_conservation::edf_throttle_from_airspeed,
};

pub struct AirspeedControl {
    pub pid: Pid<f32>,
}

impl AirspeedControl {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        let mut pid = Pid::new(0.0, MAX_AIRSPEED_METERS_PER_SECOND);
        pid.p(kp, 10.0);
        pid.i(ki, 5.0);
        pid.d(kd, 1.0);

        Self { pid }
    }

    fn update_airspeed(&mut self, current_airspeed: f32) -> f32 {
        self.pid.next_control_output(current_airspeed).output
    }

    pub fn compute_throttle(&mut self, current_airspeed: f32, air_density: f32) -> u16 {
        let throttle = self.update_airspeed(current_airspeed);
        edf_throttle_from_airspeed(throttle, air_density)
    }

    pub fn update_setpoint(&mut self, new_setpoint: f32) {
        self.pid.setpoint = new_setpoint;
    }
}
