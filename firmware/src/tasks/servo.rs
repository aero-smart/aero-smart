use crate::{executors::servo::Servo, state::DESIRED_UPDATE_SIGNAL};

use {defmt_rtt as _, panic_probe as _};

use crate::state::GLOBAL_STATE;

#[embassy_executor::task]
pub async fn servo_task(mut servo: Servo<'static>) {
    loop {
        DESIRED_UPDATE_SIGNAL.wait().await;
        let desired_angle = {
            let state = GLOBAL_STATE.lock().await;
            state.desired_servo_angle_deg
        };
        servo.set_angle_deg(desired_angle);
    }
}
