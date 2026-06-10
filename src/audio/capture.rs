use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use crossbeam_channel::Sender;
use anyhow::Result;

/// Audio capture thread that reads from input device
pub struct AudioCapture {
    stream: Option<Stream>,
    config: StreamConfig,
}

impl AudioCapture {
    pub fn new(device: &Device, config: &StreamConfig, tx: Sender<Vec<f32>>) -> Result<Self> {
        let config_clone = config.clone();
        
        let stream = device.build_input_stream(
            config,
            move |data: &cpal::InputBuffer, _: &cpal::InputCallbackInfo| {
                // Convert input samples to f32 and send to processing pipeline
                let samples: Vec<f32> = data.as_slice().iter().map(|s| *s as f32).collect();
                let _ = tx.try_send(samples); // Non-blocking send
            },
            |err| {
                tracing::error!("Audio capture error: {}", err);
            },
            None,
        )?;

        stream.play()?;
        tracing::info!("Audio capture started");

        Ok(AudioCapture {
            stream: Some(stream),
            config: config_clone,
        })
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.pause()?;
            tracing::info!("Audio capture stopped");
        }
        Ok(())
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
