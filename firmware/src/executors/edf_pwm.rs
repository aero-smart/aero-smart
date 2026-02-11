use defmt::info;
use embassy_stm32::{
    peripherals::TIM5,
    timer::{Channel, simple_pwm::SimplePwm},
};
use embassy_time::{Duration, Timer};

pub struct EdfPwm {
    pub pwm: SimplePwm<'static, TIM5>,
    /// Reserved for future use if we want to implement a elegant and smooth throttle change
    pub last_duty_cycles: [u16; 2],
}

impl EdfPwm {
    pub fn new(pwm: SimplePwm<'static, TIM5>) -> Self {
        Self {
            pwm,
            last_duty_cycles: [0; 2],
        }
    }

    fn set_duty_cycle(&mut self, channel: Channel, duty_cycle: u16) {
        let max_cycles = self.pwm.max_duty_cycle();
        info!(
            "Setting duty cycle on channel {} to {} (max {})",
            channel.index(),
            duty_cycle,
            max_cycles
        );
        self.pwm
            .channel(channel)
            .set_duty_cycle_fraction(duty_cycle, max_cycles);
    }

    fn set_duty_cycle_symmetric(&mut self, duty_cycle: u16) {
        self.set_duty_cycle(Channel::Ch1, duty_cycle);
        self.set_duty_cycle(Channel::Ch2, duty_cycle);
    }

    pub fn set_throttle_compatible(&mut self, throttle_dshot: u16) {
        if !self.pwm.ch1().is_enabled() || !self.pwm.ch2().is_enabled() {
            defmt::warn!("EDF PWM channels are not enabled. Enabling now.");
            self.enable();
        }
        let ratio = throttle_dshot as f32 / 2000.0;
        let max_duty = self.pwm.max_duty_cycle() as f32;
        let duty_cycle = (ratio * max_duty) * 0.05 + (max_duty * 0.05); // 5% min duty cycle, 10% max duty cycle
        let duty_cycle = duty_cycle as u16;
        info!(
            "Setting throttle compatible: DShot {} -> Duty Cycle {}",
            throttle_dshot, duty_cycle
        );
        self.set_duty_cycle_symmetric(duty_cycle);
    }

    pub fn stop(&mut self) {
        info!("Stopping EDF PWM");
        self.set_duty_cycle_symmetric(0);
    }

    pub fn disable(&mut self) {
        info!("Emergency stopping EDF PWM");
        self.pwm.ch1().disable();
        self.pwm.ch2().disable();
    }

    pub fn enable(&mut self) {
        info!("Enabling EDF PWM");
        self.pwm.ch1().enable();
        self.pwm.ch2().enable();
    }

    pub async fn initialize(&mut self) {
        self.enable();
        self.set_throttle_compatible(0);
        Timer::after(Duration::from_secs(5)).await;
    }

    pub async fn calibrate(&mut self) {
        self.enable();

        // Step 1: Set to zero throttle
        info!("Initializing EDF PWM: Step 1 - Setting to zero throttle");
        self.set_throttle_compatible(0);
        Timer::after(Duration::from_millis(400)).await;
        // Step 2: Send max throttle for ESC calibration
        info!("Initializing EDF PWM: Step 2 - Sending max throttle for ESC calibration");
        self.set_throttle_compatible(2000);
        Timer::after(Duration::from_millis(3600)).await;
        // Step 3: Set back to zero throttle
        info!("Initializing EDF PWM: Step 3 - Setting back to zero throttle");
        self.set_throttle_compatible(0);
        Timer::after(Duration::from_millis(4000)).await;
        info!("EDF PWM initialization complete");
    }
}
