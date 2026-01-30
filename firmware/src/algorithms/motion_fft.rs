use aerosmart_shared::serial;
use embassy_time::Instant;
use microfft::real::rfft_1024;

mod windows {
    include!("hann_window.rs");
}

#[derive(Debug, defmt::Format, Copy, Clone, PartialEq)]
pub struct VibrationMetrics {
    /// Overall vibration level
    pub rms_vibration: f32,
    /// Main problem frequency
    pub dominant_frequency_hz: f32,
    /// Severity of worst vibration
    pub peak_magnitude: f32,
}

impl From<VibrationMetrics> for serial::ImuVibrationMetrics {
    fn from(metrics: VibrationMetrics) -> Self {
        Self {
            rms_vibration: metrics.rms_vibration,
            dominant_frequency_hz: metrics.dominant_frequency_hz,
            peak_magnitude: metrics.peak_magnitude,
            time_elapsed_ms: Instant::now().as_millis(),
        }
    }
}

pub fn compute_motion_fft(input: &mut [f32; 1024]) -> [microfft::Complex32; 512] {
    for (i, sample) in input.iter_mut().enumerate() {
        let window_value = windows::HANN_WINDOW[i];
        *sample *= window_value;
    }

    let fft_output = rfft_1024(input);

    *fft_output
}

pub fn analyze_vibration(
    fft_output: &[microfft::Complex32; 512],
    sample_rate_hz: f32,
) -> VibrationMetrics {
    let mut peak_power = 0.0;
    let mut dominant_bin_index = 0;
    let mut sum_power = 0.0;

    // Single pass: find peak and accumulate power
    for (i, bin) in fft_output.iter().enumerate().skip(1) {
        let power = bin.re * bin.re + bin.im * bin.im;

        if power > peak_power {
            peak_power = power;
            dominant_bin_index = i;
        }

        sum_power += power;
    }

    let dominant_frequency_hz = (dominant_bin_index as f32 * sample_rate_hz) / 1024.0;
    let peak_magnitude = libm::sqrtf(peak_power);
    let rms_vibration = libm::sqrtf(sum_power / 511.0); // 512 bins - 1 (skipped DC)

    VibrationMetrics {
        rms_vibration,
        dominant_frequency_hz,
        peak_magnitude,
    }
}
