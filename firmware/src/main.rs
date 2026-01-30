#![no_std]
#![no_main]

pub mod algorithms;
pub mod consts;
pub mod executors;
pub mod sensors;
pub mod state;
pub mod tasks;
pub mod utils;

use crate::{
    algorithms::{airspeed::AirspeedControl, madgwick::MadgwickAhrs},
    executors::{edf::EdfDshot, servo::Servo},
    sensors::{imu_spi::ImuSpi, lidar_uart::LidarUart, pitot_i2c::Airspeed},
    tasks::*,
};

use {defmt_rtt as _, panic_probe as _};

use aerosmart_shared::serial::SerialMessage;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    adc::Adc,
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    i2c::{self, I2c},
    spi::{self, Spi},
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart::Uart,
    wdg::IndependentWatchdog,
};
use embassy_time::{Duration, TICK_HZ, Timer};
use ws2812_async::{Rgb, Ws2812};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
    USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;
    USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;
});

#[derive(defmt::Format)]
pub struct TestConfig {
    pub i2s: bool,
    pub spi_imu: bool,
    pub spi_ws2812: bool,
    pub uart_upper: bool,
    pub uart_lidar: bool,
    pub i2c: bool,
    pub pwm_edf: bool,
    pub pwm_servo: bool,
    pub wdt: bool,
    pub fft: bool,
    pub ahrs: bool,
    pub ctrl_airspeed: bool,
    pub battery_adc: bool,
    pub pressure_ads: bool,
    pub pitot_ads: bool,
    pub ads: bool,
}

fn get_stm_config() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();
    let dbgmcu = embassy_stm32::pac::DBGMCU;
    dbgmcu.cr().modify(|w| {
        w.set_dbgsleep_d1(true);
        w.set_dbgstby_d1(true);
        w.set_dbgstop_d1(true);
    });
    {
        info!("The tick frequency is {} Hz", TICK_HZ);
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(HSIPrescaler::DIV4);
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(400),
            mode: HseMode::Oscillator,
        });
        config.rcc.csi = true;
        config.rcc.hsi48 = Some(Default::default()); // needed for RNG
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV1),
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    config
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(get_stm_config());

    info!("AeroSmart Firmware Starting...");

    let config = TestConfig {
        i2s: false,
        spi_imu: false,
        spi_ws2812: false,
        uart_upper: false,
        uart_lidar: false,
        i2c: false,
        pwm_edf: false,
        pwm_servo: false,
        wdt: false,
        fft: false,
        ahrs: false,
        ctrl_airspeed: false,
        battery_adc: false,
        pressure_ads: false,
        pitot_ads: false,
        ads: false,
    };

    info!("Configuration: {:?}", config);

    let mut wdt = IndependentWatchdog::new(p.IWDG1, 20_000_000);

    wdt.unleash();

    info!("Watchdog initialized with 20s timeout");

    let mut spi_config = spi::Config::default();

    spi_config.frequency = Hertz::mhz(1);

    let spi = Spi::new(
        p.SPI1, p.PB3, p.PB5, p.PB4, p.DMA1_CH0, p.DMA1_CH1, spi_config,
    );

    info!("SPI1 initialized for IMU");

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = Hertz::khz(400);

    let i2c = I2c::new(
        p.I2C1, p.PB8, p.PB9, Irqs, p.DMA1_CH2, p.DMA1_CH3, i2c_config,
    );

    info!("I2C1 initialized for Airspeed Sensor & Barometer");

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

    let (uart_tx, uart_rx) = usart_upper.split();

    info!("USART1 initialized for upper serial communication");

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

    info!("USART3 initialized for LIDAR communication");

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

    info!("TIM1 initialized for EDF ESC control");

    let servo = PwmPin::new(p.PA0, embassy_stm32::gpio::OutputType::PushPull);

    let servo_pwm = SimplePwm::new(
        p.TIM2,
        Some(servo),
        None,
        None,
        None,
        Hertz::hz(50),
        embassy_stm32::timer::low_level::CountingMode::EdgeAlignedUp,
    );

    let servo_control = Servo::new(servo_pwm);

    info!("TIM2 initialized for Servo control");

    let ws2812_spi = Spi::new_txonly(p.SPI2, p.PD3, p.PC3, p.DMA1_CH4, spi::Config::default());
    let ws2812: Ws2812<_, Rgb, 1> = Ws2812::new(ws2812_spi);

    info!("SPI2 initialized for WS2812 LED control");

    let icm_ss = Output::new(p.PB2, Level::High, Speed::VeryHigh);
    let mut imu = ImuSpi::new(spi, icm_ss);
    let icm_drdy = ExtiInput::new(p.PB1, p.EXTI1, Pull::Up);

    info!("IMU SPI interface initialized");

    let mut sensors = Airspeed::new(i2c);
    let edf = EdfDshot::new(edf_pwm, p.DMA1_CH5);
    let pid = AirspeedControl::new(1.0, 0.1, 0.05);
    let ahrs = MadgwickAhrs::new(1_000.0, 0.033);

    info!("Airspeed sensor and EDF driver initialized");

    let adc1 = Adc::new(p.ADC1);

    wdt.pet();

    // ws2812.write([RGB8::new(0, 32, 0), RGB8::default()].into_iter()).await.ok();

    info!("System initialized, starting tasks...");

    if config.spi_imu {
        info!("Starting IMU initialization...");
        match imu.init().await {
            Ok(_) => info!("IMU initialized successfully"),
            Err(e) => defmt::panic!("Failed to initialize IMU: {:?}", e),
        }
    }

    if config.i2c {
        info!("Starting Airspeed Sensor initialization...");
        match sensors.init().await {
            Ok(_) => info!("Airspeed Sensor initialized successfully"),
            Err(e) => defmt::panic!("Failed to initialize Airspeed Sensor: {:?}", e),
        }
    }

    Timer::after(Duration::from_micros(200)).await;

    match spawner.spawn(watchdog_task(wdt)) {
        Ok(_) => info!("Watchdog task spawned"),
        Err(e) => defmt::panic!("Failed to spawn watchdog task: {:?}", e),
    }

    if config.uart_lidar {
        info!("Starting LIDAR task...");
        match spawner.spawn(lidar_task(lidar)) {
            Ok(_) => info!("LIDAR task spawned"),
            Err(e) => defmt::panic!("Failed to spawn LIDAR task: {:?}", e),
        }
    }

    if config.spi_imu {
        info!("Starting IMU task...");
        match spawner.spawn(imu_task(imu, icm_drdy, ahrs)) {
            Ok(_) => info!("IMU task spawned"),
            Err(e) => defmt::panic!("Failed to spawn IMU task: {:?}", e),
        }
    }

    if config.i2c {
        info!("Starting Airspeed task...");
        match spawner.spawn(airspeed_task(sensors)) {
            Ok(_) => info!("Airspeed task spawned"),
            Err(e) => defmt::panic!("Failed to spawn airspeed task: {:?}", e),
        }
    }

    if config.pwm_edf {
        info!("Starting EDF task...");
        match spawner.spawn(edf_task(edf, pid)) {
            Ok(_) => info!("EDF task spawned"),
            Err(e) => defmt::panic!("Failed to spawn EDF task: {:?}", e),
        }
    }

    if config.spi_ws2812 {
        info!("Starting LED task...");
        match spawner.spawn(led_task(ws2812)) {
            Ok(_) => info!("LED task spawned"),
            Err(e) => defmt::panic!("Failed to spawn LED task: {:?}", e),
        }
    }

    if config.fft {
        info!("Starting IMU FFT task...");
        match spawner.spawn(imu_fft_task()) {
            Ok(_) => info!("IMU FFT task spawned"),
            Err(e) => defmt::panic!("Failed to spawn IMU FFT task: {:?}", e),
        }
    }

    if config.pwm_servo {
        info!("Starting Airspeed Servo task...");
        match spawner.spawn(servo_task(servo_control)) {
            Ok(_) => info!("Airspeed Servo task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Airspeed Servo task: {:?}", e),
        }
    }

    if config.uart_upper {
        info!("Starting Serial UART RX task...");
        match spawner.spawn(serial_uart_rx_task(uart_rx)) {
            Ok(_) => info!("Serial UART RX task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Serial UART RX task: {:?}", e),
        }

        info!("Starting Serial UART TX task...");
        match spawner.spawn(serial_uart_tx_task(uart_tx)) {
            Ok(_) => info!("Serial UART TX task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Serial UART TX task: {:?}", e),
        }
    }

    if config.battery_adc {
        info!("Starting Battery Monitoring task...");
        match spawner.spawn(battery_task(adc1, p.PA3, p.DMA2_CH3)) {
            Ok(_) => info!("Battery Monitoring task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Battery Monitoring task: {:?}", e),
        }
    }
}
