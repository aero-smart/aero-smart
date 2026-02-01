use aerosmart_shared::serial::AcousticData;
use embassy_stm32::{peripherals::SAI1, sai::Sai};

use crate::{algorithms::acoustic_fft::compute_acoustic_fft, state::FFT_SIZE};

pub struct Audio<'a> {
    sai_rx: Sai<'a, SAI1, u32>,
    sai_tx: Sai<'a, SAI1, u32>,
}

impl<'a> Audio<'a> {
    pub fn new(sai_rx: Sai<'a, SAI1, u32>, sai_tx: Sai<'a, SAI1, u32>) -> Self {
        Audio { sai_rx, sai_tx }
    }

    pub async fn read_data(&mut self, buffer: &mut [u32]) -> Option<[u32; FFT_SIZE]> {
        let dummy = [0u32; 256]; // Dummy data to send
        self.sai_tx.write(&dummy).await.unwrap();
        self.sai_rx.read(buffer).await.unwrap();
        // Send data for FFT processing
        let mut fft_input = [0u32; FFT_SIZE];
        fft_input.copy_from_slice(&buffer[..FFT_SIZE]);

        Some(fft_input)
    }

    pub async fn analyze_audio(buffer: &mut [f32; FFT_SIZE]) -> AcousticData {
        let fft_result = compute_acoustic_fft(buffer);

        let sample_freq_hz = 44100.0; // Example sample rate

        crate::algorithms::acoustic_fft::analyze_acoustic(&fft_result, sample_freq_hz)
    }
}
