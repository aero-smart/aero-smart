/// 5.4.5 Register OxF4 "`ctrl_meas`"
///
/// The "`ctrl_meas`" register sets the pressure and temperature data acquisition options of the device. The register needs to be written after changing "ctri_hum" for the changes to become effective.
///
/// BITs:
/// - `7:5`: Controls oversampling of temperature data
/// - `4:2`: Controls oversampling of pressure data
/// - `1:0`: Controls the sensor mode of the device
#[derive(defmt::Format, Clone, Copy)]
pub struct CtrlMeas {
    osrs_t: Oversampling,
    osrs_p: Oversampling,
    mode: SensorMode,
}

/// Pressure oversampling
/// - 000: Skipped (output set to 0x80000)
/// - 001: oversampling ×1
/// - 010: oversampling ×2
/// - 011: oversampling x4
/// - 100: oversampling ×8
/// - 101, others: oversampling ×16
#[derive(defmt::Format, Clone, Copy, Default)]
pub enum Oversampling {
    #[default]
    Skipped = 0b000,
    X1 = 0b001,
    X2 = 0b010,
    X4 = 0b011,
    X8 = 0b100,
    X16 = 0b101,
}

/// Settings Mode
/// - `00`: Sleep mode
/// - `01` and `10`: Forced mode
/// - `11`: Normal mode
#[derive(defmt::Format, Clone, Copy, Default)]
pub enum SensorMode {
    #[default]
    Sleep = 0b00,
    Forced = 0b01,
    Forced2 = 0b10,
    Normal = 0b11,
}

impl CtrlMeas {
    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        byte |= (self.osrs_t as u8) << 5;
        byte |= (self.osrs_p as u8) << 2;
        byte |= self.mode as u8;
        byte
    }

    pub fn from_byte(byte: u8) -> Self {
        let osrs_t = match (byte >> 5) & 0b111 {
            0b000 => Oversampling::Skipped,
            0b001 => Oversampling::X1,
            0b010 => Oversampling::X2,
            0b011 => Oversampling::X4,
            0b100 => Oversampling::X8,
            0b101 => Oversampling::X16,
            _ => Oversampling::Skipped,
        };

        let osrs_p = match (byte >> 2) & 0b111 {
            0b000 => Oversampling::Skipped,
            0b001 => Oversampling::X1,
            0b010 => Oversampling::X2,
            0b011 => Oversampling::X4,
            0b100 => Oversampling::X8,
            0b101 => Oversampling::X16,
            _ => Oversampling::Skipped,
        };

        let mode = match byte & 0b11 {
            0b00 => SensorMode::Sleep,
            0b01 => SensorMode::Forced,
            0b10 => SensorMode::Forced2,
            0b11 => SensorMode::Normal,
            _ => SensorMode::Sleep,
        };

        CtrlMeas {
            osrs_t,
            osrs_p,
            mode,
        }
    }

    pub fn enabled() -> Self {
        CtrlMeas {
            osrs_t: Oversampling::X4,
            osrs_p: Oversampling::X4,
            mode: SensorMode::Normal,
        }
    }
}
