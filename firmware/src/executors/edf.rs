//! pwm control by Dshot protocol
//!
//! This module implements the Electric Ducted Fan (pwm) control using the Dshot protocol.
//! We use EMAX 80A Dshot ESCs for controlling the pwms, which flashes BLHeli firmware.
//! The Dshot protocol allows for digital communication with the ESCs, providing better
//! reliability and performance compared to traditional PWM signals.
use cortex_m::prelude::_embedded_hal_Pwm;
use defmt::info;
use dshot_frame::{Command, Frame, NormalDshot};
use embassy_stm32::peripherals::TIM1;
use embassy_stm32::timer::Channel::{Ch1, Ch2};
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::{Peri, peripherals};

use crate::utils::dshot::build_dshot_frame;

pub struct EdfDshot<'a> {
    /// Ch1 - Left pwm
    /// Ch2 - Right pwm
    pwm: SimplePwm<'a, TIM1>,
    /// DMA peripheral for DShot transmission
    pub dma: Peri<'static, peripherals::DMA1_CH5>,
}

#[derive(Debug, defmt::Format)]
pub enum ControlError {
    PwmError,
    DshotError,
    ExceededMaxThrottle,
}

impl<'a> EdfDshot<'a> {
    pub fn new(pwm: SimplePwm<'a, TIM1>, dma: Peri<'static, peripherals::DMA1_CH5>) -> Self {
        Self { pwm, dma }
    }

    pub async fn set_throttle_symmetric(&mut self, throttle: u16) -> Result<(), ControlError> {
        let max_duty_cycles = self.pwm.get_max_duty() as u16;

        let frame = build_dshot_frame(throttle, false, max_duty_cycles);

        // info!("Setting symmetric throttle to {}; its frame: {:?}; max duty cycles: {}", throttle, frame, max_duty_cycles);

        self.pwm.waveform_up(self.dma.reborrow(), Ch1, &frame).await;
        // info!("Setting symmetric throttle to {} [left]", throttle);
        // self.pwm.waveform_up(self.dma.reborrow(), Ch2, &frame).await;
        // info!("Setting symmetric throttle to {} [right]", throttle);
        self.pwm.set_duty(Ch1, 0);
        // self.pwm.set_duty(Ch2, 0);
        self.pwm.enable(Ch1);
        // self.pwm.enable(Ch2);

        Ok(())
    }

    pub async fn set_command_symmetric(&mut self, command: Command) -> Result<(), ControlError> {
        let frame = Frame::<NormalDshot>::command(command, false);
        let max_duty_cycles = self.pwm.get_max_duty() as u16;

        self.pwm
            .waveform_up(
                self.dma.reborrow(),
                Ch1,
                &frame.duty_cycles(max_duty_cycles),
            )
            .await;
        // self.pwm.waveform_up(self.dma.reborrow(), Ch2, &frame.duty_cycles(max_duty_cycles)).await;

        self.pwm.set_duty(Ch1, 0);
        // self.pwm.set_duty(Ch2, 0);
        self.pwm.enable(Ch1);
        // self.pwm.enable(Ch2);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), ControlError> {
        let frame = Frame::<NormalDshot>::command(Command::MotorStop, false);
        let max_duty_cycles = self.pwm.get_max_duty() as u16;

        self.pwm
            .waveform_up(
                self.dma.reborrow(),
                Ch1,
                &frame.duty_cycles(max_duty_cycles),
            )
            .await;
        // self.pwm.waveform_up(self.dma.reborrow(), Ch2, &frame.duty_cycles(max_duty_cycles)).await;

        Ok(())
    }
}
