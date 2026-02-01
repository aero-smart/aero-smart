use embassy_stm32::{
    pac::sai::vals::Mckdiv,
    rcc,
    sai::{Config, MasterClockDivider},
};
use embassy_sync::channel::Sender;

use crate::{
    sensors::audio_i2s::Audio,
    state::{AcousticFftInput, FFT_SIZE, GLOBAL_STATE},
};

pub fn sai_config_ltrr() -> (Config, Config) {
    use embassy_stm32::sai::*;
    let mclk_div = {
        let kernel_clock = rcc::frequency::<embassy_stm32::peripherals::SAI1>().0;
        let v = (kernel_clock / (44100 * 256)) as u8;
        assert!((0..=63).contains(&v), "MCLK divider out of range");
        Mckdiv::from_bits(v)
    };
    let rxconfig = {
        let mut config_rx = Config::default();
        config_rx.mode = Mode::Slave;
        config_rx.tx_rx = TxRx::Receiver;
        config_rx.sync_input = SyncInput::Internal;
        config_rx.sync_output = false;
        config_rx.clock_strobe = ClockStrobe::Rising;
        config_rx.master_clock_divider = mckdiv_to_sai_mclk_div(mclk_div);
        config_rx.stereo_mono = StereoMono::Stereo;
        config_rx.data_size = DataSize::Data24;
        config_rx.bit_order = BitOrder::MsbFirst;
        config_rx.frame_sync_polarity = FrameSyncPolarity::ActiveLow;
        config_rx.frame_sync_offset = FrameSyncOffset::BeforeFirstBit;
        config_rx.frame_length = 64;
        config_rx.frame_sync_active_level_length = embassy_stm32::sai::word::U7(32);
        config_rx.fifo_threshold = FifoThreshold::Quarter;

        config_rx
    };
    let txconfig = {
        let mut config_tx = rxconfig;
        config_tx.mode = Mode::Master;
        config_tx.tx_rx = TxRx::Transmitter;
        config_tx.sync_output = true;
        config_tx
    };
    (txconfig, rxconfig)
}

#[embassy_executor::task]
pub async fn acoustic_sampling_task(
    mut sai: Audio<'static>,
    tx: Sender<
        'static,
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        AcousticFftInput,
        2,
    >,
) {
    let mut buffer = [0u32; FFT_SIZE];

    loop {
        if let Some(fft_input) = sai.read_data(&mut buffer).await {
            tx.send(fft_input).await;
        }
    }
}

#[embassy_executor::task]
pub async fn acoustic_analysis_task(
    rx: embassy_sync::channel::Receiver<
        'static,
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        AcousticFftInput,
        2,
    >,
) {
    loop {
        let received = rx.receive().await;
        let mut buffer = {
            let mut buf = [0f32; FFT_SIZE];
            for (i, &sample) in received.iter().enumerate() {
                buf[i] = sample as f32;
            }
            buf
        };
        let result = Audio::analyze_audio(&mut buffer).await;
        {
            let mut state = GLOBAL_STATE.lock().await;
            state.acoustic_data = Some(result);
        }
    }
}

// TODO update with the latest Embassy API to avoid this conversion function
fn mckdiv_to_sai_mclk_div(mckdiv: Mckdiv) -> MasterClockDivider {
    match mckdiv {
        Mckdiv::DIV1 => MasterClockDivider::Div1,
        Mckdiv::DIV2 => MasterClockDivider::Div2,
        Mckdiv::DIV3 => MasterClockDivider::Div3,
        Mckdiv::DIV4 => MasterClockDivider::Div4,
        Mckdiv::DIV5 => MasterClockDivider::Div5,
        Mckdiv::DIV6 => MasterClockDivider::Div6,
        Mckdiv::DIV7 => MasterClockDivider::Div7,
        Mckdiv::DIV8 => MasterClockDivider::Div8,
        Mckdiv::DIV9 => MasterClockDivider::Div9,
        Mckdiv::DIV10 => MasterClockDivider::Div10,
        Mckdiv::DIV11 => MasterClockDivider::Div11,
        Mckdiv::DIV12 => MasterClockDivider::Div12,
        Mckdiv::DIV13 => MasterClockDivider::Div13,
        Mckdiv::DIV14 => MasterClockDivider::Div14,
        Mckdiv::DIV15 => MasterClockDivider::Div15,
        Mckdiv::DIV16 => MasterClockDivider::Div16,
        Mckdiv::DIV17 => MasterClockDivider::Div17,
        Mckdiv::DIV18 => MasterClockDivider::Div18,
        Mckdiv::DIV19 => MasterClockDivider::Div19,
        Mckdiv::DIV20 => MasterClockDivider::Div20,
        Mckdiv::DIV21 => MasterClockDivider::Div21,
        Mckdiv::DIV22 => MasterClockDivider::Div22,
        Mckdiv::DIV23 => MasterClockDivider::Div23,
        Mckdiv::DIV24 => MasterClockDivider::Div24,
        Mckdiv::DIV25 => MasterClockDivider::Div25,
        Mckdiv::DIV26 => MasterClockDivider::Div26,
        Mckdiv::DIV27 => MasterClockDivider::Div27,
        Mckdiv::DIV28 => MasterClockDivider::Div28,
        Mckdiv::DIV29 => MasterClockDivider::Div29,
        Mckdiv::DIV30 => MasterClockDivider::Div30,
        Mckdiv::DIV31 => MasterClockDivider::Div31,
        Mckdiv::DIV32 => MasterClockDivider::Div32,
        Mckdiv::DIV33 => MasterClockDivider::Div33,
        Mckdiv::DIV34 => MasterClockDivider::Div34,
        Mckdiv::DIV35 => MasterClockDivider::Div35,
        Mckdiv::DIV36 => MasterClockDivider::Div36,
        Mckdiv::DIV37 => MasterClockDivider::Div37,
        Mckdiv::DIV38 => MasterClockDivider::Div38,
        Mckdiv::DIV39 => MasterClockDivider::Div39,
        Mckdiv::DIV40 => MasterClockDivider::Div40,
        Mckdiv::DIV41 => MasterClockDivider::Div41,
        Mckdiv::DIV42 => MasterClockDivider::Div42,
        Mckdiv::DIV43 => MasterClockDivider::Div43,
        Mckdiv::DIV44 => MasterClockDivider::Div44,
        Mckdiv::DIV45 => MasterClockDivider::Div45,
        Mckdiv::DIV46 => MasterClockDivider::Div46,
        Mckdiv::DIV47 => MasterClockDivider::Div47,
        Mckdiv::DIV48 => MasterClockDivider::Div48,
        Mckdiv::DIV49 => MasterClockDivider::Div49,
        Mckdiv::DIV50 => MasterClockDivider::Div50,
        Mckdiv::DIV51 => MasterClockDivider::Div51,
        Mckdiv::DIV52 => MasterClockDivider::Div52,
        Mckdiv::DIV53 => MasterClockDivider::Div53,
        Mckdiv::DIV54 => MasterClockDivider::Div54,
        Mckdiv::DIV55 => MasterClockDivider::Div55,
        Mckdiv::DIV56 => MasterClockDivider::Div56,
        Mckdiv::DIV57 => MasterClockDivider::Div57,
        Mckdiv::DIV58 => MasterClockDivider::Div58,
        Mckdiv::DIV59 => MasterClockDivider::Div59,
        Mckdiv::DIV60 => MasterClockDivider::Div60,
        Mckdiv::DIV61 => MasterClockDivider::Div61,
        Mckdiv::DIV62 => MasterClockDivider::Div62,
        Mckdiv::DIV63 => MasterClockDivider::Div63,
        Mckdiv::_RESERVED_0 => MasterClockDivider::Div1,
    }
}
