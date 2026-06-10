pub mod filters;
pub mod ultrasonic;
pub mod spectral;
pub mod noise_gate;

use crate::config::Config;
use anyhow::Result;
use std::sync::Mutex;

pub use filters::FilterBank;
pub use ultrasonic::UltrasonicFilter;
pub use spectral::SpectralProcessor;
pub use noise_gate::NoiseGate;

/// Main DSP audio processor - coordinates all filtering stages
pub struct AudioProcessor {
    config: Config,
    filter_bank: Mutex<FilterBank>,
    ultrasonic_filter: Mutex<UltrasonicFilter>,
    spectral_processor: Mutex<SpectralProcessor>,
    noise_gate: Mutex<NoiseGate>,
}

impl AudioProcessor {
    pub fn new(config: Config) -> Result<Self> {
        tracing::info!("Initializing AudioProcessor");

        let filter_bank = FilterBank::new(config.sample_rate, &config.filters)?;
        let ultrasonic_filter = UltrasonicFilter::new(config.sample_rate, &config.filters)?;
        let spectral_processor = SpectralProcessor::new(config.sample_rate)?;
        let noise_gate = NoiseGate::new(config.filters.noise_gate_threshold);

        Ok(AudioProcessor {
            config,
            filter_bank: Mutex::new(filter_bank),
            ultrasonic_filter: Mutex::new(ultrasonic_filter),
            spectral_processor: Mutex::new(spectral_processor),
            noise_gate: Mutex::new(noise_gate),
        })
    }

    /// Process audio samples through the entire DSP pipeline
    pub fn process(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = samples.to_vec();

        // Stage 1: DC removal and high-pass filtering
        {
            let mut fb = self.filter_bank.lock().unwrap();
            fb.process_highpass(&mut output)?;
        }

        // Stage 2: Ultrasonic filtering (if enabled)
        if self.config.enable_ultrasonic_cancellation {
            let mut uf = self.ultrasonic_filter.lock().unwrap();
            uf.process(&mut output)?;
        }

        // Stage 3: Spectral gating for noise suppression (if enabled)
        if self.config.enable_spectral_gating {
            let mut sp = self.spectral_processor.lock().unwrap();
            sp.process(&mut output)?;
        }

        // Stage 4: Noise gate
        {
            let mut ng = self.noise_gate.lock().unwrap();
            ng.process(&mut output);
        }

        // Stage 5: Low-pass anti-aliasing filter
        {
            let mut fb = self.filter_bank.lock().unwrap();
            fb.process_lowpass(&mut output)?;
        }

        Ok(output)
    }

    pub fn update_config(&mut self, config: Config) -> Result<()> {
        tracing::info!("Updating AudioProcessor configuration");
        self.config = config.clone();

        *self.filter_bank.lock().unwrap() = FilterBank::new(config.sample_rate, &config.filters)?;
        *self.ultrasonic_filter.lock().unwrap() = UltrasonicFilter::new(config.sample_rate, &config.filters)?;
        *self.noise_gate.lock().unwrap() = NoiseGate::new(config.filters.noise_gate_threshold);

        Ok(())
    }
}
