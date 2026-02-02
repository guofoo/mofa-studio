# dora-asr-nano-mlx

MLX-based ASR (Automatic Speech Recognition) node for Dora using FunASR-Nano (SenseVoice + Qwen3-0.6B), optimized for Apple Silicon (M1/M2/M3/M4).

## Overview

This node provides multi-language speech-to-text transcription using:
- **Encoder**: SenseVoice (~221M parameters, 70 layers)
- **Decoder**: Qwen3-0.6B LLM (~751M parameters, 28 layers)
- **Backend**: MLX (Metal GPU acceleration)
- **Performance**: ~3x real-time on Apple Silicon
- **Languages**: Chinese, English, Japanese (base) or 31 languages (MLT variant)

## Requirements

- **Platform**: macOS with Apple Silicon (M1/M2/M3/M4)
- **Rust**: 1.82.0+
- **Memory**: ~3GB RAM
- **Model**: ~2GB (downloaded separately)

## Installation

### 1. Build the node

```bash
cargo build --release --manifest-path node-hub/dora-asr-nano-mlx/Cargo.toml
```

### 2. Download the model

```bash
# Create model directory
mkdir -p ~/.mofa/models

# Download from HuggingFace (choose one)
# Base model (Chinese/English/Japanese):
huggingface-cli download funaudiollm/Fun-ASR-Nano-2512 \
    --local-dir ~/.mofa/models/Fun-ASR-Nano-2512

# Or multilingual model (31 languages):
huggingface-cli download funaudiollm/Fun-ASR-MLT-Nano-2512 \
    --local-dir ~/.mofa/models/Fun-ASR-Nano-2512
```

The model directory should contain:
- `config.yaml` - Model architecture config
- `model.safetensors` - Model weights (~2GB)
- `Qwen3-0.6B/` - Tokenizer directory

## Usage

### In Dora Dataflow

```yaml
- id: asr
  build: cargo build --release --manifest-path ../../../node-hub/dora-asr-nano-mlx/Cargo.toml
  path: ../../../node-hub/dora-asr-nano-mlx/target/release/dora-asr-nano-mlx
  inputs:
    audio: mic-input/audio_segment
  outputs:
    - transcription
    - language_detected
    - processing_time
    - log
  env:
    SENSEVOICE_MODEL_DIR: ~/.mofa/models/Fun-ASR-Nano-2512
    ASR_NANO_LANGUAGE: auto
    LOG_LEVEL: INFO
```

### Inputs

| Input | Type | Description |
|-------|------|-------------|
| `audio` | PyArrow Array | PCM audio (any sample rate, auto-resampled to 16kHz) |
| `control` | String | Commands: `stats`, `cleanup`, `reset` |

### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transcription` | String | Transcribed text |
| `language_detected` | String | Detected language (zh/en/ja) |
| `processing_time` | Float | Inference duration (seconds) |
| `log` | JSON | Debug/status messages |

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SENSEVOICE_MODEL_DIR` | `~/.mofa/models/Fun-ASR-Nano-2512` | Model directory path |
| `ASR_NANO_LANGUAGE` | `auto` | Target language (zh/en/ja/auto) |
| `MIN_AUDIO_DURATION` | `0.1` | Minimum audio duration (seconds) |
| `MAX_AUDIO_DURATION` | `30.0` | Maximum audio duration (seconds) |
| `ASR_MLX_WARMUP` | `true` | Pre-initialize model on startup |
| `LOG_LEVEL` | `INFO` | Logging level |

## Performance

Benchmarks on Apple M3 Max:

| Audio Duration | Inference Time | RTF | Speed |
|----------------|----------------|-----|-------|
| 10s | ~3.3s | 0.33 | 3x real-time |
| 30s | ~10s | 0.33 | 3x real-time |
| 41s | ~13.6s | 0.33 | 3x real-time |

## Comparison with dora-asr-mlx (Paraformer)

| Aspect | dora-asr-nano-mlx | dora-asr-mlx |
|--------|-------------------|--------------|
| Model | SenseVoice + Qwen3 | Paraformer-large |
| Parameters | ~985M | ~220M |
| Languages | zh/en/ja (31 with MLT) | Chinese only |
| Speed | ~3x real-time | ~60x real-time |
| Decoding | Autoregressive (LLM) | Non-autoregressive |
| Quality | Higher | Good |
| Memory | ~3GB | ~1GB |

**Use Cases:**
- **dora-asr-mlx (Paraformer)**: Fast Chinese-only ASR with low latency
- **dora-asr-nano-mlx (SenseVoice)**: Multi-language ASR with higher accuracy

## Phase 2 Roadmap (Streaming)

Future versions will support streaming transcription:
- `StreamingContext` for chunked audio processing
- Partial results during long audio
- Lower latency for real-time applications

The funasr-nano-mlx library already has streaming support infrastructure that will be exposed in Phase 2.

## License

Apache-2.0
