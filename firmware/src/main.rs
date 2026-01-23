#![no_std]
#![no_main]

pub mod executors;
pub mod sensors;
pub mod state;
pub mod consts;

use crate::{executors::edf::EdfDshot, sensors::{imu_spi::ImuSpi, lidar_uart::LidarUart, pitot_i2c::Airspeed}};

use {defmt_rtt as _, panic_probe as _};

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, gpio::{Level, Output, Speed}, i2c::{self, I2c}, peripherals::IWDG1, spi::{self, Spi}, time::Hertz, timer::simple_pwm::{PwmPin, SimplePwm}, usart::Uart, wdg::IndependentWatchdog
};
use embassy_time::{Duration, Timer};
use ws2812_async::{Rgb, Ws2812};
use smart_leds_trait::SmartLedsWriteAsync;
use smart_leds::{RGB, RGB8};

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

    let mut led = Output::new(p.PB7, Level::High, Speed::Low);

    wdt.pet();

    let mut spi_config = spi::Config::default();

    spi_config.frequency = Hertz::mhz(1);

    let mut spi = Spi::new(
        p.SPI1, p.PB3, p.PB5, p.PB4, p.DMA1_CH0, p.DMA1_CH1, spi_config,
    );

    wdt.pet();

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = Hertz::khz(400);

    let mut i2c = I2c::new(
        p.I2C1, p.PB8, p.PB9, Irqs, p.DMA1_CH2, p.DMA1_CH3, i2c_config,
    );

    wdt.pet();

    fn uart_config_with_baud(baud_rate: u32) -> embassy_stm32::usart::Config {
        let mut config = embassy_stm32::usart::Config::default();
        config.baudrate = baud_rate;
        config
    }

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

    let mut lidar = LidarUart::new(usart_lidar);

    wdt.pet();

    let left_esc = PwmPin::new(p.PE9, embassy_stm32::gpio::OutputType::PushPull);
    let right_esc = PwmPin::new(p.PE11, embassy_stm32::gpio::OutputType::PushPull);

    let mut edf_pwm = SimplePwm::new(
        p.TIM1,
        Some(left_esc),
        Some(right_esc),
        None,
        None,
        Hertz::khz(600),
        embassy_stm32::timer::low_level::CountingMode::CenterAlignedBothInterrupts,
    );

    wdt.pet();

    let servo = PwmPin::new(p.PA0, embassy_stm32::gpio::OutputType::PushPull);

    let mut servo_pwm = SimplePwm::new(
        p.TIM2,
        Some(servo),
        None,
        None,
        None,
        Hertz::hz(50),
        embassy_stm32::timer::low_level::CountingMode::EdgeAlignedUp,
    );

    wdt.pet();

    let ws2812_spi = Spi::new_txonly(p.SPI2, p.PD3, p.PC3, p.DMA1_CH4, spi::Config::default());

    let mut ws2812: Ws2812<_, Rgb, 1> = Ws2812::new(ws2812_spi);

    ws2812.write([RGB8::new(0, 32, 0)].into_iter()).await.ok();

    wdt.pet();

    let mut icm_ss = Output::new(p.PB2, Level::High, Speed::VeryHigh);

    let mut imu = ImuSpi::new(spi, icm_ss);

    let mut sensors = Airspeed::new(i2c);

    let mut edf = EdfDshot::new(edf_pwm, p.DMA1_CH5);

    match imu.init().await {
        Ok(_) => info!("IMU initialized successfully"),
        Err(e) => defmt::panic!("Failed to initialize IMU: {:?}", e),
    }

    match spawner.spawn(feed_watchdog(wdt)) {
        Ok(_) => info!("Watchdog task spawned"),
        Err(e) => defmt::panic!("Failed to spawn watchdog task: {:?}", e),
    }

    match spawner.spawn(imu_task(imu)) {
        Ok(_) => info!("IMU task spawned"),
        Err(e) => defmt::panic!("Failed to spawn IMU task: {:?}", e),
    }

    match spawner.spawn(airspeed_task(sensors)) {
        Ok(_) => info!("Airspeed task spawned"),
        Err(e) => defmt::panic!("Failed to spawn airspeed task: {:?}", e),
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
            match sensors.read_raw_pitot().await {
                Ok((status, pressure_raw, temperature_raw)) => {
                    defmt::info!(
                        "Pitot Status: {} | Pressure Raw: {} | Temperature Raw: {}",
                        status,
                        pressure_raw,
                        temperature_raw
                    );
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
