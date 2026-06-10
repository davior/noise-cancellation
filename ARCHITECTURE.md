# Noise Cancellation Application - Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Flow                          │
└─────────────────────────────────────────────────────────────┘

UI Thread (egui)           Audio Thread              Processing Thread
      │                          │                          │
      ├─► Device Selection      │                          │
      │   Filter Controls        │                          │
      │   Status Display         │                          │
      │                          │                          │
      └──────────────────────────┼──────────────────────────┘
                                 │
                            ┌────▼────┐
                            │ Capture  │ (Input Device)
                            └────┬────┘
                                 │
                        Samples (Crossbeam Channel)
                                 │
                            ┌────▼─────────────────────┐
                            │  Audio Processing       │
                            │  Pipeline               │
                            │                         │
                            │  1. High-Pass Filter    │
                            │  2. Ultrasonic Filter   │
                            │  3. Spectral Gating     │
                            │  4. Noise Gate          │
                            │  5. Low-Pass Filter     │
                            └────┬─────────────────────┘
                                 │
                        Samples (Crossbeam Channel)
                                 │
                            ┌────▼────┐
                            │ Playback │ (Output Device)
                            └─────────┘
```

## Low-Latency Design

- **Single-buffer architecture**: Minimal queuing between capture and playback
- **Fixed-size audio frames**: 256 samples @ 48kHz = 5.3ms
- **Real-time priority**: Audio processing on dedicated thread
- **Zero-copy processing**: DSP operations work in-place
- **Target latency**: <10ms (typical: 8-12ms)

## Ultrasonic Frequency Handling

### Why It Matters
- Ultrasonic noise from devices (mice, keyboards, power supplies)
- RF interference in the 18-48kHz range
- Ultrasonic speakers/proximity sensors

### Implementation
- **Notch Filter**: Attenuates 18-48kHz band
- **Configurable Range**: Adjust `ultrasonic_lower_hz` and `ultrasonic_upper_hz`
- **Quality Factor**: Dynamic Q based on bandwidth
- **Minimal Impact**: Preserves audible range (20Hz-20kHz)

## Thread Safety

- **Audio callbacks**: Use lock-free `Sender::try_send()`
- **Config updates**: `Mutex<AudioProcessor>` protects DSP state
- **UI thread**: Independent event loop, no blocking
- **Processing thread**: Waits on channels without blocking audio threads

## Performance Targets

| Metric | Target | Typical |
|--------|--------|--------|
| CPU (single-core) | <5% | 2-3% |
| Memory (resident) | <50MB | 20-30MB |
| Latency (roundtrip) | <10ms | 8-12ms |
| Sample rate support | 44.1-192kHz | All standard rates |
