use super::{capture::AudioCapture, playback::AudioPlayback};
use crate::config::Config;
use crate::dsp::AudioProcessor;
use anyhow::Result;
use cpal::StreamConfig;
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Manages the entire audio pipeline: capture → processing → playback
pub struct AudioManager {
    config: Config,
    capture: Option<AudioCapture>,
    playback: Option<AudioPlayback>,
    processor: Arc<AudioProcessor>,
    running: Arc<AtomicBool>,
    process_thread: Option<thread::JoinHandle<()>>,
}

impl AudioManager {
    pub fn new(config: &Config) -> Result<Self> {
        tracing::info!("Initializing AudioManager with sample rate: {}", config.sample_rate);

        // Setup stream config
        let stream_config = StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(config.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(config.buffer_size as u32),
        };

        // Get audio devices
        let input_device = super::get_device_by_name(
            config.input_device.as_deref().unwrap_or("default"),
            true,
        )?;
        let output_device = super::get_device_by_name(
            config.output_device.as_deref().unwrap_or("default"),
            false,
        )?;

        // Create channels for audio data flow
        let (capture_tx, capture_rx) = unbounded::<Vec<f32>>();
        let (process_tx, process_rx) = unbounded::<Vec<f32>>();

        // Initialize DSP processor
        let processor = Arc::new(AudioProcessor::new(config.clone())?);}

        // Start capture
        let capture = AudioCapture::new(&input_device, &stream_config, capture_tx)?;

        // Start playback
        let playback = AudioPlayback::new(&output_device, &stream_config, process_rx)?;

        // Start processing thread
        let running = Arc::new(AtomicBool::new(true));
        let processor_clone = processor.clone();
        let running_clone = running.clone();

        let process_thread = thread::Builder::new()
            .name("audio-processor".to_string())
            .spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    if let Ok(samples) = capture_rx.recv() {
                        match processor_clone.process(&samples) {
                            Ok(processed) => {
                                let _ = process_tx.try_send(processed);
                            }
                            Err(e) => {
                                tracing::error!("Processing error: {}", e);
                            }
                        }
                    }
                }
            })?;

        tracing::info!("AudioManager initialized successfully");

        Ok(AudioManager {
            config: config.clone(),
            capture: Some(capture),
            playback: Some(playback),
            processor,
            running,
            process_thread: Some(process_thread),
        })
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping AudioManager");
        
        self.running.store(false, Ordering::Relaxed);

        if let Some(mut capture) = self.capture.take() {
            capture.stop()?;
        }

        if let Some(mut playback) = self.playback.take() {
            playback.stop()?;
        }

        if let Some(thread) = self.process_thread.take() {
            let _ = thread.join();
        }

        tracing::info!("AudioManager stopped");
        Ok(())
    }

    pub fn get_processor(&self) -> Arc<AudioProcessor> {
        self.processor.clone()
    }

    pub fn list_input_devices() -> Result<Vec<String>> {
        super::list_input_devices()
    }

    pub fn list_output_devices() -> Result<Vec<String>> {
        super::list_output_devices()
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
