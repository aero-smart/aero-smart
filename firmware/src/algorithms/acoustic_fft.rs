use aerosmart_shared::serial::AcousticData;
use embassy_time::Instant;

use crate::state::FFT_SIZE;

mod windows {
    include!("blackman_harris_window.rs");
}

pub fn compute_acoustic_fft(input: &mut [f32; FFT_SIZE]) -> [microfft::Complex32; FFT_SIZE / 2] {
    for (i, sample) in input.iter_mut().enumerate() {
        let window_value = windows::BLACKMAN_HARRIS_WINDOW[i];
        *sample *= window_value;
    }

    let fft_output = microfft::real::rfft_4096(input);

    *fft_output
}

pub fn analyze_acoustic(
    fft_output: &[microfft::Complex32; FFT_SIZE / 2],
    sample_rate_hz: f32,
) -> AcousticData {
    use num_traits::Float;

    let mut peak_frequency = 0.0;
    let mut peak_magnitude = 0.0;
    let mut spectral_shape = [0.0f32; 16];

    // Calculate frequency resolution
    let freq_resolution = sample_rate_hz / 1024.0;

    // Calculate magnitude spectrum and find peak
    let mut total_power = 0.0;
    let mut magnitudes = [0.0f32; 512];

    for (i, complex) in fft_output.iter().enumerate() {
        let magnitude = (complex.re * complex.re + complex.im * complex.im).sqrt();
        magnitudes[i] = magnitude;
        total_power += magnitude * magnitude;

        // Track peak frequency and magnitude
        if magnitude > peak_magnitude {
            peak_magnitude = magnitude;
            peak_frequency = i as f32 * freq_resolution;
        }
    }

    // Calculate overall SPL (Sound Pressure Level)
    // SPL = 20 * log10(RMS / reference)
    let rms = (total_power / 512.0).sqrt();
    let overall_spl = 20.0 * rms.log10().max(-100.0); // Clamp to avoid -inf

    // Calculate spectral shape (energy distribution across 16 frequency bands)
    let bins_per_band = 512 / 16;
    for (band, shape) in spectral_shape.iter_mut().enumerate() {
        let start_bin = band * bins_per_band;
        let end_bin = start_bin + bins_per_band;
        let mut band_energy = 0.0;

        for magnitude in magnitudes.iter().take(end_bin).skip(start_bin) {
            band_energy += magnitude * magnitude;
        }

        *shape = (band_energy / bins_per_band as f32).sqrt();
    }

    // Calculate turbulence index (spectral flux - rate of change in spectrum)
    // This measures roughness/variability in the acoustic signal
    let mut spectral_variation = 0.0;
    for i in 1..512 {
        let diff = magnitudes[i] - magnitudes[i - 1];
        spectral_variation += diff * diff;
    }
    let turbulence_index = (spectral_variation / 511.0).sqrt();

    AcousticData {
        overall_spl,
        peak_frequency,
        peak_magnitude,
        spectral_shape,
        turbulence_index,
        time_elapsed_ms: Instant::now().as_millis(),
    }
}
