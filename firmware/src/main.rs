#![no_std]
#![no_main]

pub mod algorithms;
pub mod consts;
pub mod executors;
pub mod sensors;
pub mod state;
pub mod utils;

use crate::{
    algorithms::{
        airspeed::AirspeedControl, madgwick::MadgwickAhrs, motion_fft::compute_motion_fft,
    },
    executors::{edf::EdfDshot, servo::Servo},
    sensors::{imu_spi::ImuSpi, lidar_uart::LidarUart, pitot_i2c::Airspeed},
    state::{
        AIRSPEED_UPDATED_SIGNAL, DESIRED_UPDATE_SIGNAL, IMU_BUFFER_FULL_SIGNAL, IMU_UPDATED_SIGNAL,
        MachineStatus, STATUS_UPDATED_SIGNAL,
    },
    utils::{
        battery::get_battery_info, magnus::density_kg_per_m3, pitot::calculate_airspeed,
        send_message,
    },
};

use {defmt_rtt as _, panic_probe as _};

use aerosmart_shared::serial::SerialMessage;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    Peri,
    adc::{Adc, AdcChannel},
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    i2c::{self, I2c},
    mode::Async,
    peripherals::{self, IWDG1},
    spi::{self, Spi},
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart::{Uart, UartRx, UartTx},
    wdg::IndependentWatchdog,
};
use embassy_time::{Duration, Instant, TICK_HZ, Timer};
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
    let mut ws2812: Ws2812<_, Rgb, 1> = Ws2812::new(ws2812_spi);

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

    match spawner.spawn(feed_watchdog(wdt)) {
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
        match spawner.spawn(airspeed_servo_task(servo_control)) {
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

#[embassy_executor::task]
async fn feed_watchdog(mut wdt: IndependentWatchdog<'static, IWDG1>) {
    loop {
        wdt.pet();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn imu_task(mut imu: ImuSpi<'static>, input: ExtiInput<'static>, mut ahrs: MadgwickAhrs) {
    loop {
        // Poll @ 1 kHz
        Timer::after(Duration::from_millis(1)).await;
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
                ahrs.update(
                    [data.accel_x, data.accel_y, data.accel_z],
                    [
                        data.gyro_x.to_radians(),
                        data.gyro_y.to_radians(),
                        data.gyro_z.to_radians(),
                    ],
                );
                {
                    let mut state = GLOBAL_STATE.lock().await;
                    let imu_head = state.imu_head;
                    state.imu_buffer[0][imu_head] = data.accel_z;
                    state.imu_buffer[1][imu_head] = data.gyro_x;
                    state.imu_buffer[2][imu_head] = data.gyro_y;
                    state.imu_head = (state.imu_head + 1) % state.imu_buffer.len();
                    if state.imu_head == 0 {
                        IMU_BUFFER_FULL_SIGNAL.signal(());
                    }
                    IMU_UPDATED_SIGNAL.signal(());
                    state.quaternion = Some(ahrs.quaternion);
                }
            }
            Err(e) => {
                defmt::error!("IMU Error: {:?}", e);
            }
        }
    }
}

#[embassy_executor::task]
/// Poll pitot tube @ 100 Hz and barometer @ 10 Hz
async fn airspeed_task(mut sensors: Airspeed<'static>) {
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
                        state.barometer_data = Some(baro_data);
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

        match status {
            MachineStatus::Running => {
                defmt::info!(
                    "EDF Control | Measured Airspeed: {} m/s | Setpoint: {} m/s",
                    measured,
                    setpoint
                );
                pid.update_setpoint(setpoint);
                let edf_airspeed = pid.compute_throttle(measured, density);
                match edf.set_throttle_symmetric(edf_airspeed).await {
                    Ok(_) => {}
                    Err(e) => {
                        defmt::error!("EDF Control Error: {:?}", e);
                        {
                            let mut state = GLOBAL_STATE.lock().await;
                            state.machine_status = MachineStatus::Error;
                            STATUS_UPDATED_SIGNAL.signal(());
                        }
                    }
                }
            }
            MachineStatus::EmergencyStop => {
                defmt::warn!("Emergency Stop Engaged! Cutting off EDF throttle.");
                match edf.stop().await {
                    Ok(_) => {}
                    Err(e) => {
                        defmt::error!("EDF Control Error during Emergency Stop: {:?}", e);
                    }
                }
            }
            _ => {}
        }
    }
}

#[embassy_executor::task]
async fn lidar_task(mut lidar: LidarUart<'static>) {
    loop {
        match lidar.poll().await {
            Ok(distance) => {
                {
                    let mut state = GLOBAL_STATE.lock().await;
                    state.lidar_data = Some(distance);
                }
                defmt::info!("LIDAR Distance: {} mm", distance);
                if distance.signal_strength < 100 {
                    defmt::warn!("LIDAR signal strength low: {}", distance.signal_strength);
                }
                if distance.distance_cm <= 30 {
                    defmt::warn!("LIDAR distance too close: {} cm", distance.distance_cm);
                    {
                        let mut state = GLOBAL_STATE.lock().await;
                        state.machine_status = MachineStatus::EmergencyStop;
                        STATUS_UPDATED_SIGNAL.signal(());
                    }
                }
            }
            Err(e) => {
                defmt::error!("LIDAR Error: {:?}", e);
            }
        }
        Timer::after(Duration::from_secs(3)).await;
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
            .write([RGB8::new(color.0, color.1, color.2), RGB8::default()].into_iter())
            .await
            .ok();
    }
}

#[embassy_executor::task]
async fn imu_fft_task() {
    loop {
        IMU_BUFFER_FULL_SIGNAL.wait().await;
        let imu_buffer = {
            let state = GLOBAL_STATE.lock().await;
            state.imu_buffer
        };
        let mut results = [None; 3];
        for axis in 0..3 {
            let mut imu_data = imu_buffer[axis];
            let fft_output = compute_motion_fft(&mut imu_data);
            let vibration_metrics =
                crate::algorithms::motion_fft::analyze_vibration(&fft_output, 1000.0);
            defmt::info!(
                "IMU Axis {} Vibration Metrics: RMS Vibration: {} | Dominant Frequency: {} Hz | Peak Magnitude: {}",
                axis,
                vibration_metrics.rms_vibration,
                vibration_metrics.dominant_frequency_hz,
                vibration_metrics.peak_magnitude
            );
            results[axis] = Some(vibration_metrics);
        }
        {
            let mut state = GLOBAL_STATE.lock().await;
            state.vibration_metrics = Some(results.map(|x| x.unwrap()));
        }
    }
}

#[embassy_executor::task]
async fn airspeed_servo_task(mut servo: Servo<'static>) {
    loop {
        DESIRED_UPDATE_SIGNAL.wait().await;
        let desired_angle = {
            let state = GLOBAL_STATE.lock().await;
            state.desired_servo_angle_deg
        };
        servo.set_angle_deg(desired_angle);
    }
}

#[embassy_executor::task]
async fn serial_uart_rx_task(mut rx: UartRx<'static, Async>) {
    use aerosmart_shared::serial::*;
    loop {
        let mut buffer = [0u8; 256];
        match rx.read(&mut buffer).await {
            Ok(len) => {
                defmt::info!("Received {} bytes over UART", len);
            }
            Err(e) => {
                defmt::error!("UART Read Error: {:?}", e);
                continue;
            }
        }
        let message = unsafe { rkyv::access_unchecked::<ArchivedSerialMessage>(&buffer) };
        {
            let mut state = GLOBAL_STATE.lock().await;
            match message {
                ArchivedSerialMessage::ThrottleConfig(ArchivedThrottleConfig { airspeed }) => {
                    state.desired_airspeed_meters_per_second = *airspeed as f32;
                }

                ArchivedSerialMessage::ServoConfig(ArchivedServoConfig { angle }) => {
                    state.desired_servo_angle_deg = *angle as f32;
                }

                ArchivedSerialMessage::SensorConfig(ArchivedSensorConfig { imu_horizontal }) => {
                    state.config = SensorConfig {
                        imu_horizontal: *imu_horizontal,
                    }
                }

                ArchivedSerialMessage::Command(command) => {
                    match command {
                        ArchivedCommand::Start => {
                            state.machine_status = MachineStatus::Running;
                        }
                        ArchivedCommand::Stop => {
                            state.machine_status = MachineStatus::Idle;
                        }
                        ArchivedCommand::Calibrate => {
                            state.machine_status = MachineStatus::Initializing;
                        }
                    }
                    STATUS_UPDATED_SIGNAL.signal(());
                }

                _ => {
                    defmt::error!("Unknown Serial Message Received");
                    state.machine_status = MachineStatus::Error;
                    STATUS_UPDATED_SIGNAL.signal(());
                }
            };
        }
    }
}

/// Send telemetry messages over UART
///
/// - Airspeed @ 10 Hz
/// - IMU quaternion @ 20 Hz
/// - Vibration metrics @ 1 Hz
/// - Barometer data @ 1 Hz
/// - LIDAR distance @ 5 Hz
#[embassy_executor::task]
async fn serial_uart_tx_task(mut tx: UartTx<'static, Async>) {
    let mut counter = 0;
    loop {
        Timer::after(Duration::from_millis(100)).await;
        counter += 1;
        use aerosmart_shared::serial::*;
        let (airspeed, imu, quat, vibration, baro, lidar) = {
            let state = GLOBAL_STATE.lock().await;
            let last_idx =
                (state.imu_head + state.imu_buffer[0].len() - 1) % state.imu_buffer[0].len();

            let imu_data: [f32; 3] = core::array::from_fn(|i| state.imu_buffer[i][last_idx]);

            (
                state.airspeed_meters_per_second,
                imu_data,
                state.quaternion,
                state.vibration_metrics,
                state.barometer_data,
                state.lidar_data,
            )
        };
        // Send airspeed @ 10 Hz
        let airspeed_message = SerialMessage::PitotAirspeedData(PitotAirspeedData {
            splitter_left: 0f32,
            splitter_right: 0f32,
            static_port: airspeed,
            time_elapsed_ms: Instant::now().as_millis(),
        });
        let (buffer, len) = send_message(airspeed_message).await;
        tx.write(&buffer[..len]).await.ok();

        // Send IMU quaternion @ 20 Hz

        if counter % 2 == 0
            && let Some(quat) = quat
        {
            let imu_message = SerialMessage::ImuData(ImuData {
                accel_z: imu[0],
                gyro_x: imu[1],
                gyro_y: imu[2],
                quad_w: quat.w,
                quad_i: quat.i,
                quad_j: quat.j,
                quad_k: quat.k,
                time_elapsed_ms: Instant::now().as_millis(),
            });
            let (buffer, len) = send_message(imu_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        // Send vibration metrics @ 1 Hz
        if counter % 10 == 0
            && let Some(vibration_metrics) = vibration
        {
            let vibration_message = SerialMessage::ImuVibrationMetrics {
                accel_z: vibration_metrics[0].into(),
                gyro_x: vibration_metrics[1].into(),
                gyro_y: vibration_metrics[2].into(),
            };
            let (buffer, len) = send_message(vibration_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        if counter % 10 == 0
            && let Some(baro_data) = baro
        {
            // Send barometer data @ 1 Hz

            let baro_message = SerialMessage::BarometerData(baro_data);
            let (buffer, len) = send_message(baro_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        if counter % 2 == 0
            && let Some(lidar_data) = lidar
        {
            // Send LIDAR data @ 5 Hz

            let lidar_message: SerialMessage = SerialMessage::LidarData(lidar_data);
            let (buffer, len) = send_message(lidar_message).await;
            tx.write(&buffer[..len]).await.ok();
        }

        if counter >= 100 {
            counter = 0;
        }
    }
}

#[embassy_executor::task]
async fn battery_task(
    mut adc: Adc<'static, peripherals::ADC1>,
    input_chan: Peri<'static, peripherals::PA3>,
    mut dma_chan: Peri<'static, peripherals::DMA2_CH3>,
) {
    let mut refchan = adc.enable_vrefint().degrade_adc();
    let mut battchan = input_chan.degrade_adc();
    loop {
        Timer::after(Duration::from_hz(1)).await;
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
        let (battery_voltage, cell_voltage, soc) = get_battery_info(battery_raw, vrefint_raw, 3.3);
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
