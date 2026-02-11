use embassy_stm32::pac::Interrupt::DFSDM1_FLT1;

use crate::consts::sensors::IMU_SAMPLE_RATE_HZ;

/// - Name: PWR_MGMT0
/// - Address: 78 (4Eh)
/// - Serial IF: R/W
/// - Reset value: 0x00
/// - Clock Domain: SCLK_UI
///
/// BITs:
/// - `7:6`: Reserved
/// - `5`: TEMP_DIS, 0 = Temperature sensor enabled, 1 = Temperature sensor disabled
/// - `4`: IDLE, 0 = Normal operation, 1 = Enter idle
/// - `3:2`: GYRO_MODE\[1:0\], Gyroscope operating mode
///    - `00` = Gyroscope Off
///    - `01` = Standby
///    - `10` = Reserved
///    - `11` = Low Noise
/// - `1:0`: ACCEL_MODE\[1:0\], Accelerometer operating mode
///    - `00` = Accelerometer Off (default)
///    - `01` = Accelerometer Off
///    - `10` = Low Power
///    - `11` = Low Noise
///
#[derive(defmt::Format, Clone, Copy)]
pub struct PwrMgmt0 {
    pub temp_dis: bool,
    pub idle: bool,
    pub gyro_mode: GyroMode,
    pub accel_mode: AccelMode,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum GyroMode {
    GyroscopeOff = 0b00,
    Standby = 0b01,
    LowNoise = 0b11,
    Reserved = 0b10,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum AccelMode {
    AccelerometerOffDefault = 0b00,
    AccelerometerOff = 0b01,
    LowPower = 0b10,
    LowNoise = 0b11,
}

impl Default for PwrMgmt0 {
    fn default() -> Self {
        PwrMgmt0 {
            temp_dis: false,
            idle: false,
            gyro_mode: GyroMode::GyroscopeOff,
            accel_mode: AccelMode::AccelerometerOffDefault,
        }
    }
}

impl PwrMgmt0 {
    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        if self.temp_dis {
            byte |= 0b0010_0000;
        }
        if self.idle {
            byte |= 0b0001_0000;
        }
        byte |= (self.gyro_mode as u8) << 2;
        byte |= self.accel_mode as u8;
        byte
    }

    pub fn from_byte(byte: u8) -> Self {
        let temp_dis = (byte & 0b0010_0000) != 0;
        let idle = (byte & 0b0001_0000) != 0;
        let gyro_mode = match (byte >> 2) & 0b11 {
            0b00 => GyroMode::GyroscopeOff,
            0b01 => GyroMode::Standby,
            0b11 => GyroMode::LowNoise,
            _ => GyroMode::Reserved,
        };
        let accel_mode = match byte & 0b11 {
            0b00 => AccelMode::AccelerometerOffDefault,
            0b01 => AccelMode::AccelerometerOff,
            0b10 => AccelMode::LowPower,
            _ => AccelMode::LowNoise,
        };

        PwrMgmt0 {
            temp_dis,
            idle,
            gyro_mode,
            accel_mode,
        }
    }

    pub fn enabled() -> Self {
        PwrMgmt0 {
            temp_dis: false,
            idle: false,
            gyro_mode: GyroMode::LowNoise,
            accel_mode: AccelMode::LowNoise,
        }
    }
}

/// - Name: GYRO_CONFIG0
/// - Address: 79 (4Fh)
/// - Serial IF: R/W
/// - Reset value: 0x06
/// - Clock Domain: SCLK_UI
///
/// BITs:
/// - `7:5`: Gyro Full Scale Select (FS_SEL)
/// - `4`: Reserved
/// - `3:0`: Gyro ODR (Output Data Rate) Selection for UI interface output
///
#[derive(defmt::Format, Clone, Copy)]
pub struct GyroConfig0 {
    pub fs_sel: GyroFullScale,
    pub odr: GyroOdr,
}

/// Full scale select for gyroscope UI interface output
/// - 000: ±2000dps (default)
/// - 001: ±1000dps
/// - 010: ±500dps
/// - 011: ±250dps
/// - 100: ±125dps
/// - 101: ±62.5dps
/// - 110: ±31.25dps
/// - 111: ±15.625dps
#[derive(defmt::Format, Clone, Copy)]
pub enum GyroFullScale {
    Dps2000 = 0b000,
    Dps1000 = 0b001,
    Dps500 = 0b010,
    Dps250 = 0b011,
    Dps125 = 0b100,
    Dps62_5 = 0b101,
    Dps31_25 = 0b110,
    Dps15_625 = 0b111,
}

/// Gyroscope ODR selection for UI interface output
/// - 0000: Reserved
/// - 0001: 32kHz
/// - 0010: 16kHz
/// - 0011: 8kHz
/// - 0100: 4kHz
/// - 0101: 2kHz
/// - 0110: 1kHz (default)
/// - 0111: 200Hz
/// - 1000: 100Hz
/// - 1001: 50Hz
/// - 1010: 25Hz
/// - 1011: 12.5Hz
/// - 1100: Reserved
/// - 1101: Reserved
/// - 1110: Reserved
/// - 1111: 500Hz
#[derive(defmt::Format, Clone, Copy)]
pub enum GyroOdr {
    Reserved0 = 0b0000,
    Khz32 = 0b0001,
    Khz16 = 0b0010,
    Khz8 = 0b0011,
    Khz4 = 0b0100,
    Khz2 = 0b0101,
    Khz1 = 0b0110,
    Hz200 = 0b0111,
    Hz100 = 0b1000,
    Hz50 = 0b1001,
    Hz25 = 0b1010,
    Hz12_5 = 0b1011,
    Reserved1 = 0b1100,
    Reserved2 = 0b1101,
    Reserved3 = 0b1110,
    Hz500 = 0b1111,
}

impl GyroOdr {
    pub fn from_hz(hz: f32) -> Self {
        match hz as u32 {
            500 => GyroOdr::Hz500,
            200 => GyroOdr::Hz200,
            100 => GyroOdr::Hz100,
            50 => GyroOdr::Hz50,
            25 => GyroOdr::Hz25,
            12 => GyroOdr::Hz12_5,
            1000 => GyroOdr::Khz1,
            2000 => GyroOdr::Khz2,
            4000 => GyroOdr::Khz4,
            8000 => GyroOdr::Khz8,
            16000 => GyroOdr::Khz16,
            32000 => GyroOdr::Khz32,
            _ => GyroOdr::Khz1, // Default to 1kHz
        }
    }

    pub fn from_khz(khz: f32) -> Self {
        match khz as u32 {
            1 => GyroOdr::Khz1,
            2 => GyroOdr::Khz2,
            4 => GyroOdr::Khz4,
            8 => GyroOdr::Khz8,
            16 => GyroOdr::Khz16,
            32 => GyroOdr::Khz32,
            _ => GyroOdr::Khz1, // Default to 1kHz
        }
    }
}

/// - Name: ACCEL_CONFIG0
/// - Address: 80 (50h)
/// - Serial IF: R/W
/// - Reset value: 0x06
/// - Clock Domain: SCLK_UI
#[derive(defmt::Format, Clone, Copy)]
pub struct AccelConfig0 {
    pub fs_sel: AccelFullScale,
    pub odr: AccelOdr,
}

/// Full scale select for accelerometer UI interface output
/// - 000: ±16g (default)
/// - 001: ±8g
/// - 010: ±4g
/// - 011: ±2g
/// - 100: Reserved
/// - 101: Reserved
/// - 110: Reserved
/// - 111: Reserved
#[derive(defmt::Format, Clone, Copy)]
pub enum AccelFullScale {
    G16 = 0b000,
    G8 = 0b001,
    G4 = 0b010,
    G2 = 0b011,
    Reserved0 = 0b100,
    Reserved1 = 0b101,
    Reserved2 = 0b110,
    Reserved3 = 0b111,
}

/// Accelerometer ODR selection for UI interface output
/// - 0000: Reserved
/// - 0001: 32kHz (LN mode)
/// - 0010: 16kHz (LN mode)
/// - 0011: 8kHz (LN mode)
/// - 0100: 4kHz (LN mode)
/// - 0101: 2kHz (LN mode)
/// - 0110: 1kHz (LN mode) (default)
/// - 0111: 200Hz (LP or LN mode)
/// - 1000: 100Hz (LP or LN mode)
/// - 1001: 50Hz (LP or LN mode)
/// - 1010: 25Hz (LP or LN mode)
/// - 1011: 12.5Hz (LP or LN mode)
/// - 1100: 6.25Hz (LP mode)
/// - 1101: 3.125Hz (LP mode)
/// - 1110: 1.5625Hz (LP mode)
/// - 1111: 500Hz (LP or LN mode)
#[derive(defmt::Format, Clone, Copy)]
pub enum AccelOdr {
    Reserved0 = 0b0000,
    Khz32 = 0b0001,
    Khz16 = 0b0010,
    Khz8 = 0b0011,
    Khz4 = 0b0100,
    Khz2 = 0b0101,
    Khz1 = 0b0110,
    Hz200 = 0b0111,
    Hz100 = 0b1000,
    Hz50 = 0b1001,
    Hz25 = 0b1010,
    Hz12_5 = 0b1011,
    Hz6_25 = 0b1100,
    Hz3_125 = 0b1101,
    Hz1_5625 = 0b1110,
    Hz500 = 0b1111,
}

impl AccelOdr {
    pub fn from_hz(hz: f32) -> Self {
        match hz as u32 {
            500 => AccelOdr::Hz500,
            200 => AccelOdr::Hz200,
            100 => AccelOdr::Hz100,
            50 => AccelOdr::Hz50,
            25 => AccelOdr::Hz25,
            12 => AccelOdr::Hz12_5,
            6 => AccelOdr::Hz6_25,
            3 => AccelOdr::Hz3_125,
            1 => AccelOdr::Hz1_5625,
            1000 => AccelOdr::Khz1,
            2000 => AccelOdr::Khz2,
            4000 => AccelOdr::Khz4,
            8000 => AccelOdr::Khz8,
            16000 => AccelOdr::Khz16,
            32000 => AccelOdr::Khz32,
            _ => AccelOdr::Khz1, // Default to 1kHz
        }
    }

    pub fn from_khz(khz: f32) -> Self {
        match khz as u32 {
            1 => AccelOdr::Khz1,
            2 => AccelOdr::Khz2,
            4 => AccelOdr::Khz4,
            8 => AccelOdr::Khz8,
            16 => AccelOdr::Khz16,
            32 => AccelOdr::Khz32,
            _ => AccelOdr::Khz1, // Default to 1kHz
        }
    }
}

impl GyroConfig0 {
    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        byte |= (self.fs_sel as u8) << 5;
        byte |= self.odr as u8;
        byte
    }

    pub fn from_byte(byte: u8) -> Self {
        let fs_sel = match (byte >> 5) & 0b111 {
            0b000 => GyroFullScale::Dps2000,
            0b001 => GyroFullScale::Dps1000,
            0b010 => GyroFullScale::Dps500,
            0b011 => GyroFullScale::Dps250,
            0b100 => GyroFullScale::Dps125,
            0b101 => GyroFullScale::Dps62_5,
            0b110 => GyroFullScale::Dps31_25,
            _ => GyroFullScale::Dps15_625,
        };
        let odr = match byte & 0b1111 {
            0b0000 => GyroOdr::Reserved0,
            0b0001 => GyroOdr::Khz32,
            0b0010 => GyroOdr::Khz16,
            0b0011 => GyroOdr::Khz8,
            0b0100 => GyroOdr::Khz4,
            0b0101 => GyroOdr::Khz2,
            0b0110 => GyroOdr::Khz1,
            0b0111 => GyroOdr::Hz200,
            0b1000 => GyroOdr::Hz100,
            0b1001 => GyroOdr::Hz50,
            0b1010 => GyroOdr::Hz25,
            0b1011 => GyroOdr::Hz12_5,
            0b1111 => GyroOdr::Hz500,
            _ => GyroOdr::Reserved0,
        };

        GyroConfig0 { fs_sel, odr }
    }
}

impl AccelConfig0 {
    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        byte |= (self.fs_sel as u8) << 5;
        byte |= self.odr as u8;
        byte
    }

    pub fn from_byte(byte: u8) -> Self {
        let fs_sel = match (byte >> 5) & 0b111 {
            0b000 => AccelFullScale::G16,
            0b001 => AccelFullScale::G8,
            0b010 => AccelFullScale::G4,
            0b011 => AccelFullScale::G2,
            0b100 => AccelFullScale::Reserved0,
            0b101 => AccelFullScale::Reserved1,
            0b110 => AccelFullScale::Reserved2,
            _ => AccelFullScale::Reserved3,
        };
        let odr = match byte & 0b1111 {
            0b0000 => AccelOdr::Reserved0,
            0b0001 => AccelOdr::Khz32,
            0b0010 => AccelOdr::Khz16,
            0b0011 => AccelOdr::Khz8,
            0b0100 => AccelOdr::Khz4,
            0b0101 => AccelOdr::Khz2,
            0b0110 => AccelOdr::Khz1,
            0b0111 => AccelOdr::Hz200,
            0b1000 => AccelOdr::Hz100,
            0b1001 => AccelOdr::Hz50,
            0b1010 => AccelOdr::Hz25,
            0b1011 => AccelOdr::Hz12_5,
            0b1100 => AccelOdr::Hz6_25,
            0b1101 => AccelOdr::Hz3_125,
            0b1110 => AccelOdr::Hz1_5625,
            0b1111 => AccelOdr::Hz500,
            _ => AccelOdr::Reserved0,
        };

        AccelConfig0 { fs_sel, odr }
    }
}

impl Default for GyroConfig0 {
    fn default() -> Self {
        GyroConfig0 {
            fs_sel: GyroFullScale::Dps250,
            odr: GyroOdr::from_hz(IMU_SAMPLE_RATE_HZ as f32),
        }
    }
}

impl Default for AccelConfig0 {
    fn default() -> Self {
        AccelConfig0 {
            fs_sel: AccelFullScale::G16,
            odr: AccelOdr::from_hz(IMU_SAMPLE_RATE_HZ as f32),
        }
    }
}

/// - Name: INT_SOURCE0
/// - Address: 101 (65h)
/// - Serial IF: R/W
/// - Reset value: 0x10
/// - Clock Domain: SCLK_UI
#[derive(defmt::Format, Clone, Copy)]
pub enum IntSource0 {
    Reserved = 0b0000_0001,
    UiFsync = 0b0000_0010,
    PllRdy = 0b0000_0100,
    ResetDone = 0b0000_1000,
    UiDrdy = 0b0001_0000,
    FifoThs = 0b0010_0000,
    FifoFull = 0b0100_0000,
    UiAgcRdy = 0b1000_0000,
}

impl IntSource0 {
    pub fn to_byte(sources: &[IntSource0]) -> u8 {
        let mut byte = 0u8;
        for source in sources {
            byte |= *source as u8;
        }
        byte
    }
}
