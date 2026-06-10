/// Adaptive noise gate - suppresses audio below a threshold
pub struct NoiseGate {
    threshold: f32,
    attack_coeff: f32,
    release_coeff: f32,
    envelope: f32,
}

impl NoiseGate {
    pub fn new(threshold_db: f32) -> Self {
        // Convert dB to linear
        let threshold = 10.0_f32.powf(threshold_db / 20.0);

        // Attack and release times (in samples at 48kHz)
        let attack_time = 0.005; // 5ms
        let release_time = 0.100; // 100ms
        let sample_rate = 48_000.0; // Default, adjust as needed

        let attack_coeff = (-2.0 * std::f32::consts::PI * attack_time * sample_rate).exp();
        let release_coeff = (-2.0 * std::f32::consts::PI * release_time * sample_rate).exp();

        NoiseGate {
            threshold,
            attack_coeff,
            release_coeff,
            envelope: 0.0,
        }
    }

    /// Process samples through the noise gate
    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            let input = sample.abs();

            // Envelope follower with attack/release
            if input > self.envelope {
                self.envelope = self.attack_coeff * self.envelope + (1.0 - self.attack_coeff) * input;
            } else {
                self.envelope = self.release_coeff * self.envelope + (1.0 - self.release_coeff) * input;
            }

            // Gate: suppress if below threshold
            if self.envelope < self.threshold {
                *sample *= self.envelope / (self.threshold + 1e-6);
            }
        }
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold = 10.0_f32.powf(threshold_db / 20.0);
    }
}
