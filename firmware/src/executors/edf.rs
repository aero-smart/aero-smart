#![allow(dead_code)]
//! pwm control by Dshot protocol
//!
//! This module implements the Electric Ducted Fan (pwm) control using the Dshot protocol.
//! We use EMAX 80A Dshot ESCs for controlling the pwms, which flashes BLHeli firmware.
//! The Dshot protocol allows for digital communication with the ESCs, providing better
//! reliability and performance compared to traditional PWM signals.
use dshot_frame::{Command, Frame, NormalDshot};
use embassy_stm32::peripherals::TIM1;
use embassy_stm32::timer::Channel::{Ch1, Ch2};
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::{Peri, peripherals};

pub struct EdfDshot<'a> {
    /// Ch1 - Left pwm
    /// Ch2 - Right pwm
    pwm: SimplePwm<'a, TIM1>,
    /// DMA peripheral for DShot transmission
    pub dma: Peri<'static, peripherals::DMA1_CH5>,
}

pub enum ControlError {
    PwmError,
    DshotError,
    ExceededMaxThrottle,
}

impl<'a> EdfDshot<'a> {
    pub fn new(pwm: SimplePwm<'a, TIM1>, dma: Peri<'static, peripherals::DMA1_CH5>) -> Self {
        Self { pwm, dma }
    }

    pub async fn set_throttle_symmetric(&mut self, throttle: f32) -> Result<(), ControlError> {
        let duty_cycle = Self::get_pwm_duty_cycle(throttle);
        let max_duty_cycles = self.pwm.max_duty_cycle();
        let frame =
            Frame::<NormalDshot>::new(duty_cycle, false).ok_or_else(|| ControlError::DshotError)?;

        self.pwm
            .waveform_up_multi_channel(
                self.dma.reborrow(),
                Ch1,
                Ch2,
                &frame.duty_cycles(max_duty_cycles),
            )
            .await;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), ControlError> {
        let frame = Frame::<NormalDshot>::command(Command::MotorStop, false);
        let max_duty_cycles = self.pwm.max_duty_cycle();
        self.pwm
            .waveform_up_multi_channel(
                self.dma.reborrow(),
                Ch1,
                Ch2,
                &frame.duty_cycles(max_duty_cycles),
            )
            .await;

        Ok(())
    }

    #[inline]
    fn get_pwm_duty_cycle(throttle: f32) -> u16 {
        // In the `dshot-frame` crate, it already defines the DSHOT range as 48-2047 for normal commands, so we map 0.0-1.0 to that range.
        const DSHOT_MAX: u16 = 2000;
        const DSHOT_MIN: u16 = 0; // minimum throttle

        let clamped_ratio = if throttle < 0.0 {
            0.0
        } else if throttle > 1.0 {
            1.0
        } else {
            throttle
        };

        (clamped_ratio * (DSHOT_MAX - DSHOT_MIN) as f32) as u16 + DSHOT_MIN
    }
}
