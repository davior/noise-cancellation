use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub high_pass_hz: f32,
    pub ultrasonic_lower_hz: f32,
    pub ultrasonic_upper_hz: f32,
    pub noise_gate_threshold: f32,
    pub spectral_floor: f32,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            high_pass_hz: 20.0,
            ultrasonic_lower_hz: 18_000.0,
            ultrasonic_upper_hz: 48_000.0,
            noise_gate_threshold: -40.0,
            spectral_floor: -80.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub sample_rate: u32,
    pub buffer_size: usize,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub filters: FilterConfig,
    pub enable_ultrasonic_cancellation: bool,
    pub enable_spectral_gating: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            buffer_size: 256,
            input_device: None,
            output_device: None,
            filters: FilterConfig::default(),
            enable_ultrasonic_cancellation: true,
            enable_spectral_gating: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = "config/config.json";
        
        if Path::new(path).exists() {
            let content = fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            tracing::warn!("Config file not found, using defaults");
            Ok(Config::default())
        }
    }

    pub fn save(&self, path: &str) -> Result<()> {
        fs::create_dir_all("config")?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
