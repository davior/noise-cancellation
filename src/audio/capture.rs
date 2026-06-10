use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use crossbeam_channel::Sender;
use anyhow::Result;

/// Audio capture thread that reads from input device
pub struct AudioCapture {
    stream: Stream,
    config: StreamConfig,
}

impl AudioCapture {
    pub fn new(device: &Device, config: &StreamConfig, tx: Sender<Vec<f32>>) -> Result<Self> {
        let stream = device.build_input_stream(
            *config,
            move |data: &[f32], _info: &cpal::InputCallbackInfo| {
                let _ = tx.try_send(data.to_vec());
            },
            move |err| {
                tracing::error!("Audio capture error: {}", err);
            },
            None,
        )?;

        stream.play()?;
        tracing::info!("Audio capture started");

        Ok(AudioCapture {
            stream,
            config: *config,
        })
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn stop(&mut self) -> Result<()> {
        self.stream.pause()?;
        tracing::info!("Audio capture stopped");
        Ok(())
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        let _ = self.stream.pause();
    }
}
