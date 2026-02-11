use crate::{consts::sensors::BATTERY_VOLTAGE_SAMPLE_RATE_HZ, utils::battery::get_battery_info};

use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::{
    Peri,
    adc::{Adc, AdcChannel},
    peripherals::{self},
};
use embassy_time::{Duration, Timer};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn battery_task(
    mut adc: Adc<'static, peripherals::ADC1>,
    input_chan: Peri<'static, peripherals::PA3>,
    mut dma_chan: Peri<'static, peripherals::DMA2_CH3>,
) {
    let mut refchan = adc.enable_vrefint().degrade_adc();
    let mut battchan = input_chan.degrade_adc();
    loop {
        Timer::after(Duration::from_hz(BATTERY_VOLTAGE_SAMPLE_RATE_HZ as u64)).await;
        let mut sum = [0u32; 2];
        // For better accuracy, average 4 samples
        for _ in 0..4 {
            let mut buffer = [0u16; 2];
            adc.read(
                dma_chan.reborrow(),
                [
                    (&mut refchan, embassy_stm32::adc::SampleTime::CYCLES810_5),
                    (&mut battchan, embassy_stm32::adc::SampleTime::CYCLES387_5),
                ]
                .into_iter(),
                &mut buffer,
            )
            .await;
            sum[0] += buffer[0] as u32;
            sum[1] += buffer[1] as u32;
        }
        let vrefint_raw = (sum[0] / 4) as u16;
        let battery_raw = (sum[1] / 4) as u16;
        // 4S LiPo, voltage divider: 5k/1k from the battery to ground
        // Vref is 3V3.
        let (battery_voltage, cell_voltage, soc) = get_battery_info(battery_raw, vrefint_raw);
        defmt::info!(
            "Battery Voltage: {} V | Cell Voltage: {} V | State of Charge: {} %",
            battery_voltage,
            cell_voltage,
            soc * 100.0
        );
        {
            let mut state = GLOBAL_STATE.lock().await;
            state.battery_voltage_volts = battery_voltage;
            state.battery_soc_percent = soc * 100.0;
        }
    }
}
