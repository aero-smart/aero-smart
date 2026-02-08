use crate::sensors::qei::QeiOperations;
use defmt::info;
use embassy_futures::select::select;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Sender};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn qei_task(
    mut qei_ops: QeiOperations<'static>,
    sender: Sender<'static, CriticalSectionRawMutex, (u16, bool, bool), 2>,
) {
    loop {
        let qei_fut = select(
            qei_ops.press_btn.wait_for_any_edge(),
            Timer::after(Duration::from_hz(20)),
        )
        .await;

        let mut btn_pressed = false;

        if qei_fut.is_first() {
            info!("Button pressed, reporting selection event");
            // Handle button press event
            btn_pressed = true;
        }

        let (position, direction, changed) = qei_ops.read_position();
        info!("QEI Position: {}, Direction: {:?}", position, direction);
        if changed || btn_pressed {
            sender.send((position, direction, btn_pressed)).await;
        }
    }
}
