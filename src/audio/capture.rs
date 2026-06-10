use cpal::traits::{DeviceTrait, StreamTrait, HostTrait};
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
        let tx = std::sync::Arc::new(std::sync::Mutex::new(tx));
        let tx_clone = tx.clone();
        
        let stream = device.build_input_stream(
            config,
            move |data: &cpal::InputBuffer, _info: &cpal::InputCallbackInfo| {
                let samples: Vec<f32> = data
                    .as_slice()
                    .iter()
                    .map(|&s| s as f32 / 32768.0)
                    .collect();
                
                if let Ok(tx) = tx_clone.lock() {
                    let _ = tx.try_send(samples);
                }
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
            config: config.clone(),
        })
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        let _ = self.stream.pause();
        tracing::info!("Audio capture stopped");
    }
}
