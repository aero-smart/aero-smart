use embassy_stm32::{peripherals::TIM2, timer::simple_pwm::SimplePwm};

pub struct Servo<'a> {
    pub pwm: SimplePwm<'a, TIM2>,
}

impl<'a> Servo<'a> {
    pub fn new(pwm: SimplePwm<'a, TIM2>) -> Self {
        Servo { pwm }
    }

    pub fn set_angle_deg(&mut self, angle_deg: f32) {
        // Assuming 0 degrees corresponds to 1ms pulse and 180 degrees to 2ms pulse
        let pulse_width_ms = 1.0 + (angle_deg / 180.0);
        let period_ms = 20.0; // Standard servo period of 20ms
        let duty_cycle = (pulse_width_ms / period_ms) * 100.0;

        self.pwm.ch1().set_duty_cycle_percent(duty_cycle as u8);
    }

    pub fn enable(&mut self) {
        self.pwm.ch1().enable();
    }
}
