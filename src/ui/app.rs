use crate::audio::AudioManager;
use crate::config::Config;
use egui::*;

pub struct App {
    audio_manager: AudioManager,
    selected_input: String,
    selected_output: String,
    input_devices: Vec<String>,
    output_devices: Vec<String>,
    
    // Filter parameters
    noise_gate_threshold: f32,
    high_pass_freq: f32,
    ultrasonic_enabled: bool,
    spectral_enabled: bool,
    
    // Status
    status_message: String,
}

impl App {
    pub fn new(audio_manager: AudioManager) -> Self {
        let input_devices = AudioManager::list_input_devices().unwrap_or_default();
        let output_devices = AudioManager::list_output_devices().unwrap_or_default();

        let selected_input = input_devices.first().cloned().unwrap_or_default();
        let selected_output = output_devices.first().cloned().unwrap_or_default();

        App {
            audio_manager,
            selected_input,
            selected_output,
            input_devices,
            output_devices,
            noise_gate_threshold: -40.0,
            high_pass_freq: 20.0,
            ultrasonic_enabled: true,
            spectral_enabled: true,
            status_message: "Running".to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎙️ Real-Time Noise Cancellation");
            ui.separator();

            // Status
            ui.label(RichText::new(format!("Status: {}", self.status_message)).color(Color32::GREEN));
            ui.separator();

            // Device Selection
            ui.group(|ui| {
                ui.label("📡 Audio Devices");
                
                ComboBox::from_label("Input Device")
                    .selected_text(&self.selected_input)
                    .show_ui(ui, |ui| {
                        for device in &self.input_devices {
                            ui.selectable_value(&mut self.selected_input, device.clone(), device);
                        }
                    });

                ComboBox::from_label("Output Device")
                    .selected_text(&self.selected_output)
                    .show_ui(ui, |ui| {
                        for device in &self.output_devices {
                            ui.selectable_value(&mut self.selected_output, device.clone(), device);
                        }
                    });
            });
            ui.separator();

            // Filter Settings
            ui.group(|ui| {
                ui.label("🎚️ Filter Settings");
                
                ui.checkbox(&mut self.ultrasonic_enabled, "Enable Ultrasonic Cancellation (>20kHz)");
                ui.checkbox(&mut self.spectral_enabled, "Enable Spectral Gating");
                
                ui.horizontal(|ui| {
                    ui.label("Noise Gate Threshold (dB):");
                    ui.slider_range(&mut self.noise_gate_threshold, -80.0..=-20.0);
                    ui.label(format!("{:.1}", self.noise_gate_threshold));
                });

                ui.horizontal(|ui| {
                    ui.label("High-Pass Cutoff (Hz):");
                    ui.slider_range(&mut self.high_pass_freq, 10.0..=200.0);
                    ui.label(format!("{:.1}", self.high_pass_freq));
                });
            });
            ui.separator();

            // Control Buttons
            if ui.button("💾 Save Configuration").clicked() {
                match Config::default().save("config/config.json") {
                    Ok(_) => self.status_message = "Configuration saved!".to_string(),
                    Err(e) => self.status_message = format!("Error saving config: {}", e),
                }
            }

            if ui.button("🔄 Reload Configuration").clicked() {
                match Config::load() {
                    Ok(_config) => self.status_message = "Configuration reloaded!".to_string(),
                    Err(e) => self.status_message = format!("Error loading config: {}", e),
                }
            }

            ui.separator();
            ui.label("⚠️ Tip: Use low buffer sizes (<512) for minimal latency");
            ui.label("💡 Select your microphone and speaker, then adjust filters as needed");
        });
    }
}
