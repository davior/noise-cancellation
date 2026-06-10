use cpal::traits::{DeviceTrait, StreamTrait, HostTrait};
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
        let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));
        let rx_clone = rx.clone();

        let stream = device.build_output_stream(
            config,
            move |output: &mut cpal::OutputBuffer, _info: &cpal::OutputCallbackInfo| {
                let out_slice = output.as_mut_slice();
                
                if let Ok(rx) = rx_clone.lock() {
                    match rx.try_recv() {
                        Ok(samples) => {
                            for (i, sample) in samples.iter().enumerate() {
                                if i < out_slice.len() {
                                    out_slice[i] = (*sample * 32767.0) as i16;
                                }
                            }
                        }
                        Err(_) => {
                            for sample in out_slice.iter_mut() {
                                *sample = 0i16;
                            }
                        }
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
            config: config.clone(),
        })
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }
}

impl Drop for AudioPlayback {
    fn drop(&mut self) {
        let _ = self.stream.pause();
        tracing::info!("Audio playback stopped");
    }
}
