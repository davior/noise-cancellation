pub mod app;
pub mod audio_monitor;

use crate::audio::AudioManager;
use anyhow::Result;

pub fn run_app(audio_manager: AudioManager) -> Result<()> {
    let native_options = eframe::NativeOptions::default();
    let app = app::App::new(audio_manager);
    
    let _ = eframe::run_native(
        "Noise Cancellation",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    );

    Ok(())
}
