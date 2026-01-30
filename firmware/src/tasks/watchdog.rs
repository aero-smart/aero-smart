use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::{peripherals::IWDG1, wdg::IndependentWatchdog};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn watchdog_task(mut wdt: IndependentWatchdog<'static, IWDG1>) {
    loop {
        wdt.pet();
        Timer::after(Duration::from_millis(1000)).await;
    }
}
