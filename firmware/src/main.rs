#![no_std]
#![no_main]

pub mod algorithms;
pub mod consts;
pub mod executors;
pub mod sensors;
pub mod state;
pub mod tasks;
#[macro_use]
pub mod utils;

use crate::{
    algorithms::{airspeed::AirspeedControl, madgwick::MadgwickAhrs},
    consts::{
        algorithms::{MADGWICK_BETA, PID_KD, PID_KI, PID_KP},
        sensors::{
            EDF_PWM_FREQUENCY_HZ, I2C_FREQ_KHZ, IMU_SAMPLE_RATE_HZ, LIDAR_BAUD_RATE,
            SERVO_PWM_FREQUENCY_HZ, UPPER_BAUD_RATE,
        },
    },
    executors::{edf_pwm::EdfPwm, servo::Servo},
    sensors::{
        audio_i2s::Audio,
        drivers::icm_42688_p::{AccelConfig0, GyroConfig0},
        imu_spi::ImuSpi,
        lidar_uart::LidarUart,
        pitot_i2c::Airspeed,
    },
    state::{AUDIO_CHANNEL, GLOBAL_STATE, SAI_BUFFER},
    tasks::*,
};

use {defmt_rtt as _, panic_probe as _};

use aerosmart_shared::serial::SerialMessage;
use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_stm32::{
    adc::Adc,
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    i2c::{self, I2c},
    rtc::{Rtc, RtcConfig},
    sai::{Sai, split_subblocks},
    spi::{self, Spi},
    time::Hertz,
    timer::simple_pwm::{PwmPin, PwmPinConfig, SimplePwm},
    usart::Uart,
    wdg::IndependentWatchdog,
};
use embassy_time::{Duration, TICK_HZ, Timer, with_timeout};
use ws2812_async::{Rgb, Ws2812};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C3_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C3>;
    I2C3_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C3>;
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
    pub analog_pressure: bool,
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
        config.rcc.pll2 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV8),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV2;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.apb3_pre = APBPrescaler::DIV2;
        config.rcc.apb4_pre = APBPrescaler::DIV2;
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
        spi_imu: true,
        spi_ws2812: false,
        uart_upper: true,
        uart_lidar: false,
        i2c: true,
        pwm_edf: true,
        pwm_servo: false,
        wdt: true,
        fft: true,
        ahrs: true,
        ctrl_airspeed: true,
        battery_adc: true,
        analog_pressure: true,
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

    fn i2c_config_with_freq(freq: Hertz) -> i2c::Config {
        let mut config = i2c::Config::default();
        config.frequency = freq;
        config
    }

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH2,
        p.DMA1_CH3,
        i2c_config_with_freq(Hertz::khz(I2C_FREQ_KHZ)),
    );

    info!("I2C1 initialized for Airspeed Sensor & Barometer");

    let i2c_adc = I2c::new(
        p.I2C3,
        p.PA8,
        p.PC9,
        Irqs,
        p.DMA2_CH4,
        p.DMA2_CH5,
        i2c_config_with_freq(Hertz::khz(I2C_FREQ_KHZ)),
    );

    info!("I2C4 initialized for ADCs (using ADS1115 chip)");

    let i2c_analog_drdy = ExtiInput::new(p.PC8, p.EXTI8, Pull::Up);

    // The `MPXV7002` hasn't been purchased and we don't have one to test with yet
    let analog_adc = sensors::adc_i2c::AdcI2c::new(
        i2c_adc,
        Some(sensors::adc_i2c::AdcConnection::Xgzp6847a {
            max_pressure_pa: 3_000.0,
        }),
        Some(sensors::adc_i2c::AdcConnection::Xgzp6847a {
            max_pressure_pa: 2_500.0,
        }),
        None,
        None,
        i2c_analog_drdy,
    );

    fn uart_config_with_baud(baud_rate: u32) -> embassy_stm32::usart::Config {
        let mut config = embassy_stm32::usart::Config::default();
        config.baudrate = baud_rate;
        config
    }

    let Ok(mut usart_upper) = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        Irqs,
        p.DMA2_CH0,
        p.DMA2_CH1,
        uart_config_with_baud(UPPER_BAUD_RATE),
    ) else {
        defmt::panic!("Failed to initialize upper USART");
    };

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());

    if config.uart_upper {
        let result = with_timeout(Duration::from_secs(5), serial_initialize(&mut usart_upper, &mut rtc)).await;
        if let Err(_) = result {
            defmt::error!("RTC synchronization via UART timed out");
            // Software reset
            cortex_m::peripheral::SCB::sys_reset();
        } else {
            info!("RTC synchronized via UART");
        }
    }

    let (uart_tx, uart_rx) = usart_upper.split();

    info!("USART1 initialized for upper serial communication");

    let Ok(usart_lidar) = Uart::new(
        p.USART3,
        p.PC11,
        p.PC10,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        uart_config_with_baud(LIDAR_BAUD_RATE),
    ) else {
        defmt::panic!("Failed to initialize LIDAR USART");
    };

    let lidar = LidarUart::new(usart_lidar);

    info!("USART3 initialized for LIDAR communication");

    fn pull_down_config() -> PwmPinConfig {
        PwmPinConfig {
            output_type: embassy_stm32::gpio::OutputType::PushPull,
            speed: Speed::VeryHigh,
            pull: Pull::Down,
        }
    }

    let left_esc_servo = PwmPin::new_with_config(p.PA0, pull_down_config());
    let right_esc_servo = PwmPin::new_with_config(p.PA1, pull_down_config());

    let edf_servo_pwm = SimplePwm::new(
        p.TIM5,
        Some(left_esc_servo),
        Some(right_esc_servo),
        None,
        None,
        Hertz::hz(EDF_PWM_FREQUENCY_HZ),
        embassy_stm32::timer::low_level::CountingMode::EdgeAlignedUp,
    );

    info!("TIM1 initialized for EDF ESC control");

    let servo = PwmPin::new(p.PA2, embassy_stm32::gpio::OutputType::PushPull);

    let servo_pwm = SimplePwm::new(
        p.TIM2,
        None,
        None,
        Some(servo),
        None,
        Hertz::hz(SERVO_PWM_FREQUENCY_HZ),
        embassy_stm32::timer::low_level::CountingMode::EdgeAlignedUp,
    );

    let servo_control = Servo::new(servo_pwm);

    info!("TIM2 initialized for Servo control");

    let ws2812_spi = Spi::new_txonly(p.SPI2, p.PD3, p.PC3, p.DMA1_CH4, spi::Config::default());
    let ws2812: Ws2812<_, Rgb, 1> = Ws2812::new(ws2812_spi);

    info!("SPI2 initialized for WS2812 LED control");

    let icm_ss = Output::new(p.PB2, Level::High, Speed::VeryHigh);
    let mut imu = ImuSpi::new(spi, icm_ss, AccelConfig0::default(), GyroConfig0::default());
    let icm_drdy = ExtiInput::new(p.PB1, p.EXTI1, Pull::Up);

    info!("IMU SPI interface initialized");

    let mut sensors = Airspeed::new(i2c);
    let edf = EdfPwm::new(edf_servo_pwm);
    let pid = AirspeedControl::new(0.0, PID_KP, PID_KI, PID_KD);
    let ahrs = MadgwickAhrs::new(IMU_SAMPLE_RATE_HZ as f32, MADGWICK_BETA);

    info!("Airspeed sensor and EDF driver initialized");

    let adc1 = Adc::new(p.ADC1);

    let (sai_tx_conf, sai_rx_conf) = sai_config_ltrr();

    let (sai_rx_p, sai_tx_p) = split_subblocks(p.SAI1);

    let (sai_tx_buffer, sai_rx_buffer): (&mut [u32], &mut [u32]) = unsafe {
        let buf = &mut *core::ptr::addr_of_mut!(SAI_BUFFER);
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        let slice = core::slice::from_raw_parts_mut(ptr, len);
        slice.split_at_mut(512)
    };

    let sai_rx = Sai::new_asynchronous_with_mclk(
        sai_rx_p,
        p.PE5,
        p.PE6,
        p.PE4,
        p.PE2,
        p.DMA2_CH7,
        sai_rx_buffer,
        sai_rx_conf,
    );

    let sai_tx = Sai::new_synchronous(sai_tx_p, p.PE3, p.DMA2_CH6, sai_tx_buffer, sai_tx_conf);

    let microphone = Audio::new(sai_rx, sai_tx);

    wdt.pet();

    // ws2812.write([RGB8::new(0, 32, 0), RGB8::default()].into_iter()).await.ok();

    info!("System initialized, starting tasks...");

    match (config.spi_imu, config.i2c) {
        (true, true) => match join::join(imu.init(), sensors.init()).await {
            (Ok(_), Ok(_)) => info!("IMU and Airspeed Sensor initialized successfully"),
            (Err(e1), Err(e2)) => defmt::panic!(
                "Failed to initialize IMU and Airspeed Sensor: {:?}, {:?}",
                e1,
                e2
            ),
            (Err(e), _) => defmt::panic!("Failed to initialize IMU: {:?}", e),
            (_, Err(e)) => defmt::panic!("Failed to initialize Airspeed Sensor: {:?}", e),
        },
        (true, false) => {
            info!("Starting IMU initialization...");
            match imu.init().await {
                Ok(_) => info!("IMU initialized successfully"),
                Err(e) => defmt::panic!("Failed to initialize IMU: {:?}", e),
            }
        }
        (false, true) => {
            info!("Starting Airspeed Sensor initialization...");
            match sensors.init().await {
                Ok(_) => info!("Airspeed Sensor initialized successfully"),
                Err(e) => defmt::panic!("Failed to initialize Airspeed Sensor: {:?}", e),
            }
        }
        (false, false) => { /* No sensors to initialize */ }
    }

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

    if config.analog_pressure {
        info!("Starting Analog Pressure Sensor task...");
        match spawner.spawn(analog_pressure_task(analog_adc)) {
            Ok(_) => info!("Analog Pressure Sensor task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Analog Pressure Sensor task: {:?}", e),
        }
    }

    if config.i2s {
        info!("Starting Acoustic Sampling task...");
        match spawner.spawn(acoustic_sampling_task(microphone, AUDIO_CHANNEL.sender())) {
            Ok(_) => info!("Acoustic Audio task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Acoustic Audio task: {:?}", e),
        }
        info!("Starting Acoustic Analysis task...");
        match spawner.spawn(acoustic_analysis_task(AUDIO_CHANNEL.receiver())) {
            Ok(_) => info!("Acoustic Analysis task spawned"),
            Err(e) => defmt::panic!("Failed to spawn Acoustic Analysis task: {:?}", e),
        }
    }

    spawner.spawn(test_pid_task()).unwrap();
}

#[embassy_executor::task]
async fn test_pid_task() {
    Timer::after(Duration::from_secs(5)).await;
    {
        let mut state = GLOBAL_STATE.lock().await;
        state.desired_airspeed_meters_per_second = 13.5;
        state.machine_status = state::MachineStatus::Running;
    }
}
