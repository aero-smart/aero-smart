use crate::state::STATUS_UPDATED_SIGNAL;

use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::{mode::Async, spi::Spi};
use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWriteAsync;
use ws2812_async::{Rgb, Ws2812};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn led_task(mut ws2812: Ws2812<Spi<'static, Async>, Rgb, 1>) {
    loop {
        STATUS_UPDATED_SIGNAL.wait().await;
        let color = {
            let state = GLOBAL_STATE.lock().await;
            state.machine_status.display_led()
        };
        ws2812
            .write([RGB8::new(color.0, color.1, color.2), RGB8::default()].into_iter())
            .await
            .ok();
    }
}
