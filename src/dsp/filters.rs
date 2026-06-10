use crate::config::FilterConfig;
use anyhow::Result;

/// Simple IIR biquad filter for efficient real-time processing
#[derive(Clone)]
struct BiquadFilter {
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    z1: f32, z2: f32,
}

impl BiquadFilter {
    /// Create a high-pass filter using bilinear transform
    fn high_pass(cutoff_hz: f32, sample_rate: u32) -> Self {
        let wc = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate as f32;
        let sw = wc.sin();
        let cw = wc.cos();
        let a = sw / (2.0 * 0.707); // Q = 0.707 for Butterworth response

        let a0 = 1.0 + a;
        let a1 = -2.0 * cw / a0;
        let a2 = (1.0 - a) / a0;

        let b0 = (1.0 + cw) / (2.0 * a0);
        let b1 = -(1.0 + cw) / a0;
        let b2 = (1.0 + cw) / (2.0 * a0);

        BiquadFilter {
            b0, b1, b2, a1, a2,
            z1: 0.0, z2: 0.0,
        }
    }

    /// Create a low-pass filter using bilinear transform
    fn low_pass(cutoff_hz: f32, sample_rate: u32) -> Self {
        let wc = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate as f32;
        let sw = wc.sin();
        let cw = wc.cos();
        let a = sw / (2.0 * 0.707); // Q = 0.707 for Butterworth response

        let a0 = 1.0 + a;
        let a1 = -2.0 * cw / a0;
        let a2 = (1.0 - a) / a0;

        let b0 = (1.0 - cw) / (2.0 * a0);
        let b1 = (1.0 - cw) / a0;
        let b2 = (1.0 - cw) / (2.0 * a0);

        BiquadFilter {
            b0, b1, b2, a1, a2,
            z1: 0.0, z2: 0.0,
        }
    }

    /// Process a single sample through the filter
    fn process(&mut self, sample: f32) -> f32 {
        let output = self.b0 * sample + self.z1;
        self.z1 = self.b1 * sample + self.z2 - self.a1 * output;
        self.z2 = self.b2 * sample - self.a2 * output;
        output
    }
}

/// Bank of filters for multi-stage noise cancellation
pub struct FilterBank {
    high_pass: BiquadFilter,
    low_pass: BiquadFilter,
}

impl FilterBank {
    pub fn new(sample_rate: u32, config: &FilterConfig) -> Result<Self> {
        Ok(FilterBank {
            high_pass: BiquadFilter::high_pass(config.high_pass_hz, sample_rate),
            low_pass: BiquadFilter::low_pass(20_000.0, sample_rate), // Anti-aliasing at Nyquist
        })
    }

    pub fn process_highpass(&mut self, samples: &mut [f32]) -> Result<()> {
        for sample in samples.iter_mut() {
            *sample = self.high_pass.process(*sample);
        }
        Ok(())
    }

    pub fn process_lowpass(&mut self, samples: &mut [f32]) -> Result<()> {
        for sample in samples.iter_mut() {
            *sample = self.low_pass.process(*sample);
        }
        Ok(())
    }
}
