#![deny(clippy::all)]

use std::fs::File;

use aerosmart_shared::serial::*;

use ts_rs::TS;

use std::fmt::Write;

macro_rules! export_schema {
    ($($structure:ident),*) => {
        {
            let mut index_string = String::new();
            $(
                $structure::export_all().expect(concat!("Failed to export TS definitions for ", stringify!($structure)));
                write!(
                    index_string,
                    "export type {{ {} }} from './{}';\n",
                    stringify!($structure),
                    stringify!($structure)
                ).expect("Failed to write to index string");
            )*
            index_string
        }
    };
}

fn main() {
    let ts_index = export_schema!(
        ThrottleConfig,
        SerialMessage,
        SensorConfig,
        AcknowledgementConfig,
        Command,
        AcknowledgementData,
        PitotAirspeedData,
        ImuData,
        AcousticData,
        BarometerData,
        AnalogPressureSensorData,
        QeiData,
        ImuVibrationMetrics,
        LidarData,
        BatteryData
    );

    File::create("bindings/generated/index.ts")
        .and_then(|mut file| std::io::Write::write_all(&mut file, ts_index.as_bytes()))
        .expect("Failed to write index.ts file");
}
