# MoFA ASR

Dedicated ASR (Automatic Speech Recognition) app for MoFA Studio with support for MLX-based ASR engines.

## Features

- **Audio Panel**: Mic toggle, AEC (Acoustic Echo Cancellation), audio device selection
- **Chat Window**: Real-time transcription display with model attribution
- **Hero Panel**: Start/stop dataflow, system resource monitoring
- **Settings Page**: Model selection, language configuration, duration limits

## Supported ASR Models

| Model | Engine | Languages | Speed | Use Case |
|-------|--------|-----------|-------|----------|
| **Paraformer** | dora-asr-mlx | Chinese only | ~60x real-time | Low-latency Chinese ASR |
| **SenseVoice** | dora-funasr-nano-mlx | zh/en/ja | ~3x real-time | Multilingual ASR |

## Dataflow Configurations

### Single Model

```
mofa-mic-input --> asr --> mofa-transcription-display --> UI
```

- `dataflow/asr-mlx.yml` - Paraformer only (Chinese, fastest)
- `dataflow/asr-nano-mlx.yml` - SenseVoice only (multilingual)

### Dual Model (Compare)

```
                    +--> asr-paraformer --+
mofa-mic-input -----+                     +--> mofa-transcription-display --> UI
                    +--> asr-sensevoice --+
```

- `dataflow/asr-dual.yml` - Both models simultaneously

## Settings

### Model Selection
- **Paraformer**: Chinese only, extremely fast (~60x real-time)
- **SenseVoice**: Supports Chinese, English, Japanese with auto-detection
- **Both**: Run both models and compare results

### SenseVoice Language
- `auto` - Auto-detect language (default)
- `zh` - Chinese
- `en` - English
- `ja` - Japanese

### Audio Duration
- **Minimum**: 0.1s - 5.0s (ignore audio shorter than this)
- **Maximum**: 5.0s - 60.0s (chunk audio longer than this)

### Advanced
- **Warmup**: Pre-initialize model on startup for faster first inference
- **Custom Dataflow**: Select custom YAML dataflow file

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PARAFORMER_MODEL_DIR` | `~/.mofa/models/paraformer-large-mlx` | Paraformer model path |
| `SENSEVOICE_MODEL_DIR` | `~/.mofa/models/Fun-ASR-Nano-2512` | SenseVoice model path |
| `ASR_NANO_LANGUAGE` | `auto` | SenseVoice language setting |
| `MIN_AUDIO_DURATION` | `0.1` | Minimum audio duration (seconds) |
| `MAX_AUDIO_DURATION` | `30.0` | Maximum audio duration (seconds) |
| `ASR_MLX_WARMUP` | `true` | Pre-initialize model |
| `LOG_LEVEL` | `INFO` | Log verbosity |

## Architecture

```
apps/mofa-asr/
├── Cargo.toml
├── README.md
├── dataflow/
│   ├── asr-mlx.yml           # Paraformer dataflow
│   ├── asr-nano-mlx.yml      # SenseVoice dataflow
│   └── asr-dual.yml          # Dual model dataflow
└── src/
    ├── lib.rs                # App registration
    ├── dora_integration.rs   # Dora bridge management
    └── screen/
        ├── mod.rs            # Main screen widget
        ├── design.rs         # UI layout (live_design!)
        ├── audio_controls.rs # Mic, AEC, device handling
        ├── chat_panel.rs     # Transcription display
        └── settings.rs       # Settings persistence
```

## UI Layout

```
+-----------------------------------------------------------+
|  MofaHero (Start/Stop, CPU, Memory, GPU stats)            |
+-----------------------------------------------------------+
|  [Transcription Tab] [Settings Tab]                       |
+-----------------------------------------------------------+
|                                                           |
|  +-----------------------------------------------------+  |
|  |  ASR Transcription                            [Copy]|  |
|  |  -------------------------------------------------- |  |
|  |  **SenseVoice** (14:32:15) [zh]:                    |  |
|  |  Hello, this is a test transcription.               |  |
|  |                                                      |  |
|  |  **Paraformer** (14:32:14):                         |  |
|  |  你好，这是一个测试转录。                             |  |
|  +-----------------------------------------------------+  |
|                                                           |
+-----------------------------------------------------------+
|  [Mic] [=====     ] [AEC]  Input: [v]  Output: [v]       |
|                            Active Model: SenseVoice       |
+-----------------------------------------------------------+
```

## Usage

1. Select ASR model in Settings tab
2. Configure language (for SenseVoice)
3. Click Start in Hero panel
4. Speak into microphone
5. View transcriptions in real-time

## Dependencies

- `mofa-ui` - Shared UI components (MicButton, AecButton, LedMeter, ChatPanel)
- `mofa-dora-bridge` - Dora dataflow integration
- `mofa-widgets` - Base theme and MofaApp trait
- `mofa-settings` - Preferences persistence
