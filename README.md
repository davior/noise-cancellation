# Real-Time Noise Cancellation

A cross-platform real-time noise cancellation application built in Rust, with ultra-low latency and support for ultrasonic frequency filtering.

## Features

- 🎙️ **Real-time audio capture** from any system microphone
- 🔊 **Live playback** through selected audio output device
- 🎯 **Intelligent noise suppression** with adaptive filtering
- 📊 **Ultrasonic filtering** (handles frequencies >20kHz)
- ⚡ **Ultra-low latency** (target: <10ms roundtrip)
- 🖥️ **Cross-platform** (Windows, macOS, Linux)
- 🎨 **Simple GUI** for device and parameter selection
- 🔧 **Configurable** audio parameters and filter settings

## Quick Start

### Prerequisites

- Rust 1.70+
- For Linux: ALSA development libraries
  ```bash
  sudo apt-get install libasound2-dev
  ```

### Build & Run

```bash
cargo run --release
```

## Architecture Overview

```
Input Device → High-Pass Filter → Ultrasonic Filter → Spectral Gate → Noise Gate → Low-Pass Filter → Output Device
```

### Key Components

1. **High-Pass Filter**: Removes DC and very low frequencies (cutoff: 20Hz)
2. **Ultrasonic Filter**: Targets and suppresses frequencies >18kHz (up to 48kHz)
3. **Spectral Gating**: FFT-based noise suppression
4. **Noise Gate**: Suppresses audio below threshold (-40dB default)
5. **Low-Pass Filter**: Anti-aliasing before output (20kHz)

## Configuration

Edit `config/config.json` to customize:

```json
{
  "sample_rate": 48000,
  "buffer_size": 256,
  "filters": {
    "high_pass_hz": 20.0,
    "ultrasonic_lower_hz": 18000.0,
    "ultrasonic_upper_hz": 48000.0,
    "noise_gate_threshold": -40.0
  }
}
```

## Performance

- **Latency**: 5-12ms roundtrip (capture → processing → playback)
- **CPU**: <5% single-core usage
- **Memory**: <50MB RAM
- **Sample Rates**: 44.1kHz, 48kHz, 96kHz, 192kHz

## Ultrasonic Noise Cancellation

This application specifically targets ultrasonic noise from:
- Device interference (mice, keyboards, power supplies)
- RF interference (18-48kHz range)
- Proximity sensors and ultrasonic speakers

Ultrasonic filtering is particularly important for:
- Clean audio capture in noisy environments
- Removing inaudible but problematic interference
- Maintaining audio fidelity for human hearing (20Hz-20kHz)

## Platform Support

- ✅ Windows (WASAPI backend)
- ✅ macOS (CoreAudio backend)
- ✅ Linux (ALSA/PulseAudio backend)

## Development

### Build
```bash
cargo build --release
```

### Test
```bash
cargo test --release
```

### Lint
```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## Future Enhancements

- [ ] Machine learning-based noise profiles
- [ ] Multi-mic beamforming
- [ ] Real-time spectrum visualization
- [ ] VST/AU plugin support
- [ ] Configuration profiles
- [ ] GPU acceleration (CUDA/Metal)

## License

MIT

## References

- [WebRTC Noise Suppression](https://webrtc.github.io/samples/src/content/getusermedia/dsp/)
- [Digital Signal Processing](https://www.dspguide.com/)
- [Real-time Audio Programming](https://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nothing)
