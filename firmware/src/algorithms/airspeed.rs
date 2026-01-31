use nalgebra::clamp;
use pid::Pid;

use crate::{
    algorithms::airspeed_filter::AirspeedFilter,
    utils::mass_conservation::edf_throttle_from_airspeed,
};

#[cfg(doc)]
extern crate aquamarine;

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum AirspeedState {
    Reaching,
    Approaching,
    Cruising,
}

impl AirspeedState {
    pub fn from_airspeed(airspeed: f32, setpoint: f32) -> Self {
        let error = airspeed - setpoint;
        let abs_error = libm::fabsf(error) / setpoint;

        if abs_error < 0.2 {
            AirspeedState::Cruising
        } else if abs_error < 0.05 {
            AirspeedState::Approaching
        } else {
            AirspeedState::Reaching
        }
    }
}

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
    pub setpoint: f32,
    pub filter: AirspeedFilter,
    pub feedback_gain_max_percent: f32,
}

impl AirspeedControl {
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32) -> Self {
        let mut pid = Pid::new(0.0, 50.0);
        pid.p(kp, 10.0);
        pid.i(ki, 5.0);
        pid.d(kd, 1.0);

        Self {
            pid,
            setpoint,
            filter: AirspeedFilter::new(1.0f32, 0.001f32, 0.1f32),
            feedback_gain_max_percent: 0.2f32,
        }
    }

    fn update_airspeed(&mut self, current_airspeed: f32) -> f32 {
        let filtered_airspeed = self.filter.update(current_airspeed);
        self.pid
            .next_control_output(filtered_airspeed - self.setpoint)
            .output
    }

    /// The feedback gain is calculated as follows:
    /// - If the airspeed is Reaching, use no feedback (0%)
    /// - If the airspeed is Approaching, use sigmoid from 0% to max feedback gain: $\mathrm{gain} = \frac{G_{max}}{1 + e^{-k|v_{error}|}}$
    /// - If the airspeed is Cruising, use max feedback gain
    pub fn compute_throttle(
        &mut self,
        current_airspeed: f32,
        air_density: f32,
        voltage_v: f32,
    ) -> u16 {
        let feedforward = edf_throttle_from_airspeed(self.setpoint, air_density, voltage_v);
        let feedback_correction = self.update_airspeed(current_airspeed);
        let feedback = feedback_correction / self.pid.output_limit * 2000.0; // Scale to 0-2000 range
        let reaching_state = AirspeedState::from_airspeed(current_airspeed, self.setpoint);
        let feedback_gain = match reaching_state {
            AirspeedState::Reaching => 0f32,
            AirspeedState::Approaching => {
                let error = libm::fabsf(current_airspeed - self.setpoint) / self.setpoint;
                let k = 10f32; // Steepness of the sigmoid
                self.feedback_gain_max_percent / (1f32 + libm::expf(-k * (error - 0.05f32)))
            }
            AirspeedState::Cruising => self.feedback_gain_max_percent,
        };
        let adjusted_throttle =
            feedforward as f32 * (1.0 - feedback_gain) + feedback * feedback_gain;
        let clamped_throttle = clamp(adjusted_throttle, 0.0, 2000.0);
        clamped_throttle as u16
    }

    pub fn update_setpoint(&mut self, new_setpoint: f32) {
        self.setpoint = new_setpoint;
    }
}
