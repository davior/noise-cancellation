/// Real-time audio level monitoring and visualization
pub struct AudioMonitor {
    peak_level: f32,
    rms_level: f32,
    sample_count: usize,
}

impl AudioMonitor {
    pub fn new() -> Self {
        AudioMonitor {
            peak_level: 0.0,
            rms_level: 0.0,
            sample_count: 0,
        }
    }

    /// Update monitor with new audio samples
    pub fn update(&mut self, samples: &[f32]) {
        // Peak detection
        for sample in samples {
            let abs_sample = sample.abs();
            if abs_sample > self.peak_level {
                self.peak_level = abs_sample;
            }
        }

        // RMS calculation
        let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
        self.rms_level = (sum_squares / samples.len() as f32).sqrt();
        
        self.sample_count += samples.len();

        // Decay peak level over time
        if self.sample_count % 48_000 == 0 {
            self.peak_level *= 0.95;
        }
    }

    pub fn peak_db(&self) -> f32 {
        20.0 * self.peak_level.max(1e-6).log10()
    }

    pub fn rms_db(&self) -> f32 {
        20.0 * self.rms_level.max(1e-6).log10()
    }
}
