//! Airspeed filter using a Kalman filter approach.

#[derive(Debug, Clone, Copy, defmt::Format)]
pub struct AirspeedFilter {
    pub state: f32,
    q: f32,
    p: f32,
    r: f32,
}

impl AirspeedFilter {
    pub fn new(initial_state: f32, process_variance: f32, measurement_variance: f32) -> Self {
        AirspeedFilter {
            state: initial_state,
            q: process_variance,
            p: 1.0,
            r: measurement_variance,
        }
    }

    pub fn update(&mut self, measurement: f32) -> f32 {
        // Prediction step
        self.p += self.q;

        // Kalman Gain
        let k = self.p / (self.p + self.r);

        // Update step
        self.state += k * (measurement - self.state);
        self.p *= 1.0 - k;

        self.state
    }
}
