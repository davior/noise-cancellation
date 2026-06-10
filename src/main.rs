mod audio;
mod config;
mod dsp;
mod ui;

use anyhow::Result;
use tracing_subscriber;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::load()?;
    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize audio system
    let audio_manager = audio::AudioManager::new(&config)?;
    tracing::info!("Audio system initialized");

    // Run UI (blocking)
    ui::run_app(audio_manager)?;

    Ok(())
}
