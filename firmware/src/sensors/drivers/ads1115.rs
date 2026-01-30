/// - Name: ADS1115_CONFIG
/// - Address: 0x01 (Configuration Register)
/// - I2C Address: 0x48 (ADDR pin to GND)
/// - Serial IF: I2C
/// - Reset value: 0x8583
///
/// BITs (16-bit register):
/// - `15`: OS (Operational Status/Single-shot conversion start)
///    - `0` = No effect
///    - `1` = Start single conversion
/// - `14:12`: MUX[2:0] (Input multiplexer configuration)
///    - `000` = AIN0 - AIN1 (differential)
///    - `001` = AIN0 - AIN3 (differential)
///    - `010` = AIN1 - AIN3 (differential)
///    - `011` = AIN2 - AIN3 (differential)
///    - `100` = AIN0 - GND (single-ended)
///    - `101` = AIN1 - GND (single-ended)
///    - `110` = AIN2 - GND (single-ended)
///    - `111` = AIN3 - GND (single-ended)
/// - `11:9`: PGA[2:0] (Programmable gain amplifier configuration)
///    - `000` = ±6.144V
///    - `001` = ±4.096V
///    - `010` = ±2.048V (default)
///    - `011` = ±1.024V
///    - `100` = ±0.512V
///    - `101` = ±0.256V
///    - `110` = ±0.256V
///    - `111` = ±0.256V
/// - `8`: MODE (Device operating mode)
///    - `0` = Continuous conversion
///    - `1` = Single-shot (default)
/// - `7:5`: DR[2:0] (Data rate)
///    - `000` = 8 SPS
///    - `001` = 16 SPS
///    - `010` = 32 SPS
///    - `011` = 64 SPS
///    - `100` = 128 SPS (default)
///    - `101` = 250 SPS
///    - `110` = 475 SPS
///    - `111` = 860 SPS
/// - `4`: COMP_MODE (Comparator mode)
/// - `3`: COMP_POL (Comparator polarity)
/// - `2`: COMP_LAT (Latching comparator)
/// - `1:0`: COMP_QUE[1:0] (Comparator queue)
///    - `11` = Disable comparator (default)
///
#[derive(defmt::Format, Clone, Copy)]
pub struct Ads1115Config {
    pub os: bool,
    pub mux: AdcMux,
    pub pga: AdcPga,
    pub mode: AdcMode,
    pub data_rate: AdcDataRate,
    pub comp_queue: AdcCompQueue,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum AdcMux {
    Ain0Ain1 = 0b000,
    Ain0Ain3 = 0b001,
    Ain1Ain3 = 0b010,
    Ain2Ain3 = 0b011,
    Ain0Gnd = 0b100,
    Ain1Gnd = 0b101,
    Ain2Gnd = 0b110,
    Ain3Gnd = 0b111,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum AdcPga {
    V6_144 = 0b000,
    V4_096 = 0b001,
    V2_048 = 0b010,
    V1_024 = 0b011,
    V0_512 = 0b100,
    V0_256_1 = 0b101,
    V0_256_2 = 0b110,
    V0_256_3 = 0b111,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum AdcMode {
    Continuous = 0b0,
    SingleShot = 0b1,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum AdcDataRate {
    Sps8 = 0b000,
    Sps16 = 0b001,
    Sps32 = 0b010,
    Sps64 = 0b011,
    Sps128 = 0b100,
    Sps250 = 0b101,
    Sps475 = 0b110,
    Sps860 = 0b111,
}

#[derive(defmt::Format, Clone, Copy)]
pub enum AdcCompQueue {
    After1 = 0b00,
    After2 = 0b01,
    After4 = 0b10,
    Disable = 0b11,
}

impl Default for Ads1115Config {
    fn default() -> Self {
        Ads1115Config {
            os: false,
            mux: AdcMux::Ain0Gnd,
            pga: AdcPga::V2_048,
            mode: AdcMode::SingleShot,
            data_rate: AdcDataRate::Sps128,
            comp_queue: AdcCompQueue::Disable,
        }
    }
}

impl Ads1115Config {
    pub fn to_bytes(&self) -> [u8; 2] {
        let mut config: u16 = 0;

        if self.os {
            config |= 0b1000_0000_0000_0000;
        }
        config |= (self.mux as u16) << 12;
        config |= (self.pga as u16) << 9;
        config |= (self.mode as u16) << 8;
        config |= (self.data_rate as u16) << 5;
        config |= self.comp_queue as u16;

        [(config >> 8) as u8, config as u8]
    }

    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        let config = ((bytes[0] as u16) << 8) | (bytes[1] as u16);

        let os = (config & 0x8000) != 0;
        let mux = match (config >> 12) & 0b111 {
            0b000 => AdcMux::Ain0Ain1,
            0b001 => AdcMux::Ain0Ain3,
            0b010 => AdcMux::Ain1Ain3,
            0b011 => AdcMux::Ain2Ain3,
            0b100 => AdcMux::Ain0Gnd,
            0b101 => AdcMux::Ain1Gnd,
            0b110 => AdcMux::Ain2Gnd,
            _ => AdcMux::Ain3Gnd,
        };
        let pga = match (config >> 9) & 0b111 {
            0b000 => AdcPga::V6_144,
            0b001 => AdcPga::V4_096,
            0b010 => AdcPga::V2_048,
            0b011 => AdcPga::V1_024,
            0b100 => AdcPga::V0_512,
            0b101 => AdcPga::V0_256_1,
            0b110 => AdcPga::V0_256_2,
            _ => AdcPga::V0_256_3,
        };
        let mode = if (config & 0x0100) != 0 {
            AdcMode::SingleShot
        } else {
            AdcMode::Continuous
        };
        let data_rate = match (config >> 5) & 0b111 {
            0b000 => AdcDataRate::Sps8,
            0b001 => AdcDataRate::Sps16,
            0b010 => AdcDataRate::Sps32,
            0b011 => AdcDataRate::Sps64,
            0b100 => AdcDataRate::Sps128,
            0b101 => AdcDataRate::Sps250,
            0b110 => AdcDataRate::Sps475,
            _ => AdcDataRate::Sps860,
        };
        let comp_queue = match config & 0b11 {
            0b00 => AdcCompQueue::After1,
            0b01 => AdcCompQueue::After2,
            0b10 => AdcCompQueue::After4,
            _ => AdcCompQueue::Disable,
        };

        Ads1115Config {
            os,
            mux,
            pga,
            mode,
            data_rate,
            comp_queue,
        }
    }

    pub fn chan(channel: usize) -> Self {
        let mut config = Ads1115Config::default();
        config.mux = match channel {
            0 => AdcMux::Ain0Gnd,
            1 => AdcMux::Ain1Gnd,
            2 => AdcMux::Ain2Gnd,
            3 => AdcMux::Ain3Gnd,
            _ => AdcMux::Ain0Gnd,
        };
        config
    }
}

impl AdcPga {
    pub fn lsb_mv(&self) -> f32 {
        match self {
            AdcPga::V6_144 => 0.1875,
            AdcPga::V4_096 => 0.125,
            AdcPga::V2_048 => 0.0625,
            AdcPga::V1_024 => 0.03125,
            AdcPga::V0_512 => 0.015625,
            AdcPga::V0_256_1 | AdcPga::V0_256_2 | AdcPga::V0_256_3 => 0.0078125,
        }
    }

    pub fn to_voltage(&self, adc_value: i16) -> f32 {
        (adc_value as f32) * self.lsb_mv() / 1000.0
    }
}
