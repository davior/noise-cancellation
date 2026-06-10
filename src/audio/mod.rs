pub mod capture;
pub mod playback;
pub mod manager;

pub use manager::AudioManager;

use anyhow::Result;
use cpal::Device;
use cpal::traits::{HostTrait, DeviceTrait};

/// List all available audio input devices
pub fn list_input_devices() -> Result<Vec<String>> {
    let host = cpal::default_host();
    let mut devices = Vec::new();
    
    for device in host.input_devices()? {
        if let Ok(desc) = device.description() {
            devices.push(desc.name().to_string());
        }
    }
    
    Ok(devices)
}

/// List all available audio output devices
pub fn list_output_devices() -> Result<Vec<String>> {
    let host = cpal::default_host();
    let mut devices = Vec::new();
    
    for device in host.output_devices()? {
        if let Ok(desc) = device.description() {
            devices.push(desc.name().to_string());
        }
    }
    
    Ok(devices)
}

/// Get a device by name
pub fn get_device_by_name(name: &str, is_input: bool) -> Result<Device> {
    let host = cpal::default_host();
    let devices = if is_input {
        host.input_devices()?
    } else {
        host.output_devices()?
    };
    
    for device in devices {
        if let Ok(desc) = device.description() {
            if desc.name() == name {
                return Ok(device);
            }
        }
    }
    
    // Fallback to default device
    if is_input {
        host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))
    } else {
        host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))
    }
}
