use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use crossbeam_channel::Receiver;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Audio playback thread that writes to output device
pub struct AudioPlayback {
    stream: Option<Stream>,
    config: StreamConfig,
    has_data: Arc<AtomicBool>,
}

impl AudioPlayback {
    pub fn new(device: &Device, config: &StreamConfig, rx: Receiver<Vec<f32>>) -> Result<Self> {
        let config_clone = config.clone();
        let has_data = Arc::new(AtomicBool::new(false));
        let has_data_clone = has_data.clone();

        let stream = device.build_output_stream(
            config,
            move |output: &mut cpal::OutputBuffer, _: &cpal::OutputCallbackInfo| {
                match rx.try_recv() {
                    Ok(samples) => {
                        let out_slice = output.as_mut_slice();
                        for (i, sample) in samples.iter().enumerate() {
                            if i < out_slice.len() {
                                out_slice[i] = *sample as i16;
                            }
                        }
                        has_data_clone.store(true, Ordering::Relaxed);
                    }
                    Err(_) => {
                        // No data available, output silence
                        for sample in output.as_mut_slice() {
                            *sample = 0i16;
                        }
                        has_data_clone.store(false, Ordering::Relaxed);
                    }
                }
            },
            |err| {
                tracing::error!("Audio playback error: {}", err);
            },
        )?;

        stream.play()?;
        tracing::info!("Audio playback started");

        Ok(AudioPlayback {
            stream: Some(stream),
            config: config_clone,
            has_data,
        })
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn is_active(&self) -> bool {
        self.has_data.load(Ordering::Relaxed)
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.pause()?;
            tracing::info!("Audio playback stopped");
        }
        Ok(())
    }
}

impl Drop for AudioPlayback {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
