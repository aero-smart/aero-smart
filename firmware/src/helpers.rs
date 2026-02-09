pub mod dshot {
    use crate::executors::edf::EdfDshot;
    use defmt::{info, warn};
    use dshot_frame::Command;
    use embassy_futures::select::{Either, select};
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use embassy_sync::channel::{Channel, Receiver, Sender};
    use embassy_sync::mutex::Mutex;
    use embassy_time::{Duration, Instant, Ticker, Timer};

    use crate::state::GLOBAL_STATE;

    #[embassy_executor::task]
    pub async fn dshot_test_task(mut edf: EdfDshot<'static>) {
        let mut ticker = Ticker::every(Duration::from_hz(2_400));
        loop {
            edf.set_throttle_symmetric(LEFT_THROTTLE.lock().await.clone())
                .await
                .unwrap();
            ticker.next().await;
        }
    }

    #[embassy_executor::task]
    pub async fn dshot_set_task() {
        // Sine curve: 10 seconds period, min 100, max 700.
        let period = Duration::from_secs(120);
        let min_throttle = 200u16;
        let max_throttle = 800u16;

        info!(
            "Starting DShot set task: period: {:?}, min: {}, max: {}",
            period, min_throttle, max_throttle
        );

        use num_traits::Float;

        loop {
            let start = Instant::now();
            info!("Beginning throttle sine wave cycle");
            loop {
                let elapsed = Instant::now() - start;
                let t = elapsed.as_millis() as f32 / period.as_millis() as f32;
                let sine_value = (t * 2.0 * core::f32::consts::PI).sin();
                let throttle = ((sine_value + 1.0) / 2.0 * (max_throttle - min_throttle) as f32)
                    as u16
                    + min_throttle;
                info!(
                    "Sending throttle command: {} at elapsed {:?}",
                    throttle, elapsed
                );
                LEFT_THROTTLE.lock().await.clone_from(&throttle);
                RIGHT_THROTTLE.lock().await.clone_from(&throttle);
                Timer::after(Duration::from_secs(10)).await;
                if elapsed >= period {
                    break;
                }
            }
        }
    }

    pub static LEFT_THROTTLE: Mutex<CriticalSectionRawMutex, u16> = Mutex::new(400);
    pub static RIGHT_THROTTLE: Mutex<CriticalSectionRawMutex, u16> = Mutex::new(0);
}
