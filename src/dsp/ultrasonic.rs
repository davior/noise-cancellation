use crate::config::FilterConfig;
use anyhow::Result;

/// Specialized filter for ultrasonic frequency cancellation (>20kHz)
/// Implements a bandpass filter to isolate and suppress ultrasonic noise
pub struct UltrasonicFilter {
    lower_cutoff: f32,
    upper_cutoff: f32,
    sample_rate: u32,
    // State variables for cascaded biquads
    z1_lower: f32, z2_lower: f32,
    z1_upper: f32, z2_upper: f32,
}

impl UltrasonicFilter {
    pub fn new(sample_rate: u32, config: &FilterConfig) -> Result<Self> {
        // Validate frequencies
        let nyquist = sample_rate as f32 / 2.0;
        
        if config.ultrasonic_upper_hz > nyquist {
            tracing::warn!(
                "Ultrasonic upper frequency {} Hz exceeds Nyquist {}",
                config.ultrasonic_upper_hz,
                nyquist
            );
        }

        Ok(UltrasonicFilter {
            lower_cutoff: config.ultrasonic_lower_hz,
            upper_cutoff: config.ultrasonic_upper_hz.min(nyquist * 0.95),
            sample_rate,
            z1_lower: 0.0,
            z2_lower: 0.0,
            z1_upper: 0.0,
            z2_upper: 0.0,
        })
    }

    /// Process audio through ultrasonic notch filtering
    /// This attenuates frequencies in the ultrasonic range while preserving audible content
    pub fn process(&mut self, samples: &mut [f32]) -> Result<()> {
        for sample in samples.iter_mut() {
            // Apply notch filter to suppress ultrasonic frequencies
            *sample = self.apply_notch(*sample);
        }
        Ok(())
    }

    /// Simplified notch filter for ultrasonic suppression
    fn apply_notch(&mut self, sample: f32) -> f32 {
        let center_freq = (self.lower_cutoff + self.upper_cutoff) / 2.0;
        let bandwidth = self.upper_cutoff - self.lower_cutoff;
        let q = center_freq / bandwidth;

        // Notch filter coefficients
        let wc = 2.0 * std::f32::consts::PI * center_freq / self.sample_rate as f32;
        let sw = wc.sin();
        let cw = wc.cos();
        let alpha = sw / (2.0 * q);

        let b0 = 1.0;
        let b1 = -2.0 * cw;
        let b2 = 1.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cw / a0;
        let a2 = (1.0 - alpha) / a0;

        // Difference equation
        let output = b0 * sample + self.z1_lower;
        self.z1_lower = b1 * sample + self.z2_lower - a1 * output;
        self.z2_lower = b2 * sample - a2 * output;

        output
    }
}
