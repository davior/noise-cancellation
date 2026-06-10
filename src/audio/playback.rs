use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use crossbeam_channel::Receiver;
use anyhow::Result;

/// Audio playback thread that writes to output device
pub struct AudioPlayback {
    stream: Stream,
    config: StreamConfig,
}

impl AudioPlayback {
    pub fn new(device: &Device, config: &StreamConfig, rx: Receiver<Vec<f32>>) -> Result<Self> {
        let stream = device.build_output_stream(
            *config,
            move |output: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                match rx.try_recv() {
                    Ok(samples) => {
                        let n = samples.len().min(output.len());
                        output[..n].copy_from_slice(&samples[..n]);
                        output[n..].fill(0.0);
                    }
                    Err(_) => {
                        output.fill(0.0);
                    }
                }
            },
            move |err| {
                tracing::error!("Audio playback error: {}", err);
            },
            None,
        )?;

        stream.play()?;
        tracing::info!("Audio playback started");

        Ok(AudioPlayback {
            stream,
            config: *config,
        })
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn stop(&mut self) -> Result<()> {
        self.stream.pause()?;
        tracing::info!("Audio playback stopped");
        Ok(())
    }
}

impl Drop for AudioPlayback {
    fn drop(&mut self) {
        let _ = self.stream.pause();
    }
}
