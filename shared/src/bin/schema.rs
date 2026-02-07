#![deny(clippy::all)]

use aerosmart_shared::serial::*;
use aerosmart_shared::sql::*;

use ts_rs::TS;

fn main() {
    ThrottleConfig::export_all().expect("Failed to export TS definitions for ThrottleConfig");
    SerialMessage::export_all().expect("Failed to export TS definitions for SerialMessage");
    SensorConfig::export_all().expect("Failed to export TS definitions for SensorConfig");
    AcknowledgementConfig::export_all()
        .expect("Failed to export TS definitions for AcknowledgementConfig");
    Command::export_all().expect("Failed to export TS definitions for Command");
    AcknowledgementData::export_all()
        .expect("Failed to export TS definitions for AcknowledgementData");
    PitotAirspeedData::export_all().expect("Failed to export TS definitions for PitotAirspeedData");
    ImuData::export_all().expect("Failed to export TS definitions for ImuData");
    AcousticData::export_all().expect("Failed to export TS definitions for AcousticData");
    BarometerData::export_all().expect("Failed to export TS definitions for BarometerData");
    AnalogPressureSensorData::export_all()
        .expect("Failed to export TS definitions for AnalogPressureSensorData");
    QeiData::export_all().expect("Failed to export TS definitions for QeiData");
    ImuVibrationMetrics::export_all()
        .expect("Failed to export TS definitions for ImuVibrationMetrics");
    LidarData::export_all().expect("Failed to export TS definitions for LidarData");
    BatteryData::export_all().expect("Failed to export TS definitions for BatteryData");
    AnalogPressureSensorData::export_all()
        .expect("Failed to export TS definitions for AnalogPressureSensorData");
    SerialMessage::export_all().expect("Failed to export TS definitions for SerialMessage");
}
