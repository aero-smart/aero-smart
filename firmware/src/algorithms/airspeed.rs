use pid::Pid;

use crate::{
    consts::MAX_AIRSPEED_METERS_PER_SECOND, utils::mass_conservation::edf_throttle_from_airspeed,
};

#[cfg(doc)]
extern crate aquamarine;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Here is a diagram of the AirspeedControl structure:
///
/// ```mermaid
/// graph LR
///     %% Input
///     Vdesired[Desired Airspeed]
///     
///     %% Feedforward path
///     FF[Feedforward]
///     
///     %% Feedback path
///     Sum1((Σ))
///     PID[PID Controller]
///     
///     %% Summation
///     Sum2((Σ))
///     
///     %% Plant
///     Plant[ESC/EDF/Tunnel]
///     
///     %% Output
///     Vactual[Actual Airspeed]
///     
///     %% Sensor
///     Sensor[4525DO Sensor]
///     
///     %% Connections - Feedforward
///     Vdesired --> FF
///     FF --> Sum2
///     
///     %% Connections - Feedback
///     Vdesired --> Sum1
///     Sensor --> Sum1
///     Sum1 --> PID
///     PID --> Sum2
///     
///     %% Connections - Plant
///     Sum2 --> Plant
///     Plant --> Vactual
///     
///     %% Feedback loop
///     Vactual --> Sensor
/// ```
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

    #[inline]
    /// Feedforward airspeed calculation
    ///
    /// The feedforward airspeed is calculated based on the desired airspeed
    /// and the characteristics of the propulsion system.
    ///
    /// $$
    /// \mathrm{Thrust} = \dot m \cdot \Delta v
    /// $$
    fn feedforward_airspeed(&self) -> f32 {
        0f32
    }
}
