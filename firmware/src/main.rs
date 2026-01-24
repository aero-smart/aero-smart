#![no_std]
#![no_main]

pub mod consts;
pub mod executors;
pub mod sensors;
pub mod state;
pub mod utils;

use crate::{
    executors::{airspeed::AirspeedControl, edf::EdfDshot},
    sensors::{imu_spi::ImuSpi, lidar_uart::LidarUart, pitot_i2c::Airspeed},
    state::{AIRSPEED_UPDATED_SIGNAL, MachineStatus, STATUS_UPDATED_SIGNAL},
    utils::{magnus::density_kg_per_m3, pitot::calculate_airspeed},
};

use {defmt_rtt as _, panic_probe as _};

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    i2c::{self, I2c},
    mode::Async,
    peripherals::IWDG1,
    spi::{self, Spi},
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart::Uart,
    wdg::IndependentWatchdog,
};
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWriteAsync;
use ws2812_async::{Rgb, Ws2812};

use state::GLOBAL_STATE;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
    USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;
    USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut wdt = IndependentWatchdog::new(p.IWDG1, 20_000_000);

    wdt.unleash();

    let mut spi_config = spi::Config::default();

    spi_config.frequency = Hertz::mhz(1);

    let spi = Spi::new(
        p.SPI1, p.PB3, p.PB5, p.PB4, p.DMA1_CH0, p.DMA1_CH1, spi_config,
    );

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = Hertz::khz(400);

    let i2c = I2c::new(
        p.I2C1, p.PB8, p.PB9, Irqs, p.DMA1_CH2, p.DMA1_CH3, i2c_config,
    );

    fn uart_config_with_baud(baud_rate: u32) -> embassy_stm32::usart::Config {
        let mut config = embassy_stm32::usart::Config::default();
        config.baudrate = baud_rate;
        config
    }

    // TODO: serial communication
    let Ok(usart_upper) = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        Irqs,
        p.DMA2_CH0,
        p.DMA2_CH1,
        uart_config_with_baud(915200),
    ) else {
        defmt::panic!("Failed to initialize upper USART");
    };

    let Ok(usart_lidar) = Uart::new(
        p.USART3,
        p.PC11,
        p.PC10,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        uart_config_with_baud(115200),
    ) else {
        defmt::panic!("Failed to initialize LIDAR USART");
    };

    let lidar = LidarUart::new(usart_lidar);

    let left_esc = PwmPin::new(p.PE9, embassy_stm32::gpio::OutputType::PushPull);
    let right_esc = PwmPin::new(p.PE11, embassy_stm32::gpio::OutputType::PushPull);

    let edf_pwm = SimplePwm::new(
        p.TIM1,
        Some(left_esc),
        Some(right_esc),
        None,
        None,
        Hertz::khz(600),
        embassy_stm32::timer::low_level::CountingMode::CenterAlignedBothInterrupts,
    );

    // TODO
    // let servo = PwmPin::new(p.PA0, embassy_stm32::gpio::OutputType::PushPull);

    // let servo_pwm = SimplePwm::new(
    //     p.TIM2,
    //     Some(servo),
    //     None,
    //     None,
    //     None,
    //     Hertz::hz(50),
    //     embassy_stm32::timer::low_level::CountingMode::EdgeAlignedUp,
    // );

    let ws2812_spi = Spi::new_txonly(p.SPI2, p.PD3, p.PC3, p.DMA1_CH4, spi::Config::default());

    let mut ws2812: Ws2812<_, Rgb, 1> = Ws2812::new(ws2812_spi);

    let icm_ss = Output::new(p.PB2, Level::High, Speed::VeryHigh);

    let mut imu = ImuSpi::new(spi, icm_ss);

    let sensors = Airspeed::new(i2c);

    let edf = EdfDshot::new(edf_pwm, p.DMA1_CH5);

    let pid = AirspeedControl::new(1.0, 0.1, 0.05);

    wdt.pet();

    ws2812.write([RGB8::new(0, 32, 0)].into_iter()).await.ok();

    match imu.init().await {
        Ok(_) => info!("IMU initialized successfully"),
        Err(e) => defmt::panic!("Failed to initialize IMU: {:?}", e),
    }

    match spawner.spawn(feed_watchdog(wdt)) {
        Ok(_) => info!("Watchdog task spawned"),
        Err(e) => defmt::panic!("Failed to spawn watchdog task: {:?}", e),
    }

    match spawner.spawn(lidar_task(lidar)) {
        Ok(_) => info!("LIDAR task spawned"),
        Err(e) => defmt::panic!("Failed to spawn LIDAR task: {:?}", e),
    }

    match spawner.spawn(imu_task(imu)) {
        Ok(_) => info!("IMU task spawned"),
        Err(e) => defmt::panic!("Failed to spawn IMU task: {:?}", e),
    }

    match spawner.spawn(airspeed_task(sensors)) {
        Ok(_) => info!("Airspeed task spawned"),
        Err(e) => defmt::panic!("Failed to spawn airspeed task: {:?}", e),
    }

    match spawner.spawn(edf_task(edf, pid)) {
        Ok(_) => info!("EDF task spawned"),
        Err(e) => defmt::panic!("Failed to spawn EDF task: {:?}", e),
    }
}

#[embassy_executor::task]
async fn feed_watchdog(mut wdt: IndependentWatchdog<'static, IWDG1>) {
    loop {
        wdt.pet();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn imu_task(mut imu: ImuSpi<'static>) {
    loop {
        match imu.poll().await {
            Ok(data) => {
                defmt::info!(
                    "Accel: x={} y={} z={} | Gyro: x={} y={} z={}",
                    data.accel_x,
                    data.accel_y,
                    data.accel_z,
                    data.gyro_x,
                    data.gyro_y,
                    data.gyro_z
                );
            }
            Err(e) => {
                defmt::error!("IMU Error: {:?}", e);
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
/// Poll pitot tube @ 100 Hz and barometer @ 10 Hz
async fn airspeed_task(mut sensors: Airspeed<'static>) {
    loop {
        let mut counter = 0;
        loop {
            match sensors.read_pitot().await {
                Ok((status, pressure_raw, temperature_raw)) => {
                    defmt::info!(
                        "Pitot Status: {} | Pressure Raw: {} | Temperature Raw: {}",
                        status,
                        pressure_raw,
                        temperature_raw
                    );
                    {
                        let mut state = GLOBAL_STATE.lock().await;
                        let airspeed =
                            calculate_airspeed(pressure_raw, state.air_density_kg_per_cubic_meter);
                        state.airspeed_meters_per_second = airspeed;
                        defmt::info!("Calculated Airspeed: {} m/s", airspeed);
                    }
                    AIRSPEED_UPDATED_SIGNAL.signal(());
                }
                Err(e) => {
                    defmt::error!("Airspeed Sensor Error: {:?}", e);
                }
            }

            counter += 1;
            if counter >= 10 {
                match sensors.read_barometer().await {
                    Ok(baro_data) => {
                        defmt::info!(
                            "Barometer Pressure: {} Pa | Temperature: {} °C | Humidity: {} %",
                            baro_data.pressure_pa,
                            baro_data.temperature_c,
                            baro_data.humidity_percent
                        );
                        {
                            let mut state = GLOBAL_STATE.lock().await;
                            let density = density_kg_per_m3(
                                baro_data.pressure_pa,
                                baro_data.temperature_c,
                                baro_data.humidity_percent,
                            );
                            state.air_density_kg_per_cubic_meter = density;
                            defmt::info!("Updated Air Density: {} kg/m^3", density);
                        }
                    }
                    Err(e) => {
                        defmt::error!("Barometer Error: {:?}", e);
                    }
                }
                counter = 0;
            }

            Timer::after(Duration::from_millis(10)).await;
        }
    }
}

#[embassy_executor::task]
async fn edf_task(mut edf: EdfDshot<'static>, mut pid: AirspeedControl) {
    loop {
        AIRSPEED_UPDATED_SIGNAL.wait().await;
        let (measured, setpoint, status, density) = {
            let state = GLOBAL_STATE.lock().await;
            (
                state.airspeed_meters_per_second,
                state.desired_airspeed_meters_per_second,
                state.machine_status,
                state.air_density_kg_per_cubic_meter,
            )
        };

        if matches!(status, MachineStatus::Running) {
            pid.update_setpoint(setpoint);
            let edf_airspeed = pid.compute_throttle(measured, density);
            match edf.set_throttle_symmetric(edf_airspeed).await {
                Ok(_) => {}
                Err(e) => {
                    defmt::error!("EDF Control Error: {:?}", e);
                    {
                        let mut state = GLOBAL_STATE.lock().await;
                        state.machine_status = MachineStatus::Error;
                    }
                }
            }
        }
    }
}

#[embassy_executor::task]
async fn lidar_task(mut lidar: LidarUart<'static>) {
    loop {
        match lidar.poll().await {
            Ok(distance) => {
                defmt::info!("LIDAR Distance: {} mm", distance);
                if distance.signal_strength < 100 {
                    defmt::warn!("LIDAR signal strength low: {}", distance.signal_strength);
                }
                if distance.distance_cm <= 30 {
                    defmt::warn!("LIDAR distance too close: {} cm", distance.distance_cm);
                    {
                        let mut state = GLOBAL_STATE.lock().await;
                        state.machine_status = MachineStatus::EmergencyStop;
                    }
                }
            }
            Err(e) => {
                defmt::error!("LIDAR Error: {:?}", e);
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn led_task(mut ws2812: Ws2812<Spi<'static, Async>, Rgb, 1>) {
    loop {
        STATUS_UPDATED_SIGNAL.wait().await;
        let color = {
            let state = GLOBAL_STATE.lock().await;
            state.machine_status.display_led()
        };
        ws2812
            .write([RGB8::new(color.0, color.1, color.2)].into_iter())
            .await
            .ok();
    }
}
