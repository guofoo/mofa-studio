# dora-asr-mlx

MLX-based ASR (Automatic Speech Recognition) node for Dora using FunASR Paraformer model, optimized for Apple Silicon (M1/M2/M3/M4).

## Overview

This node provides fast Chinese speech-to-text transcription using:
- **Model**: Paraformer-large (~220M parameters)
- **Backend**: MLX (Metal GPU acceleration)
- **Performance**: 50-75x real-time on Apple Silicon
- **Language**: Chinese only

## Requirements

- **Platform**: macOS with Apple Silicon (M1/M2/M3/M4)
- **Rust**: 1.82.0+
- **Model**: ~800MB (downloaded separately)

## Installation

### 1. Build the node

```bash
cargo build --release --manifest-path node-hub/dora-asr-mlx/Cargo.toml
```

### 2. Download the model

```bash
# Create model directory
mkdir -p ~/.mofa/models/paraformer-large-mlx

# Download from HuggingFace
huggingface-cli download funaudiollm/paraformer-large-mlx \
    --local-dir ~/.mofa/models/paraformer-large-mlx
```

The model directory should contain:
- `paraformer.safetensors` - Model weights
- `am.mvn` - CMVN normalization parameters
- `tokens.txt` - Vocabulary (8404 tokens)

## Usage

### In Dora Dataflow

```yaml
- id: asr
  build: cargo build --release --manifest-path ../../../node-hub/dora-asr-mlx/Cargo.toml
  path: ../../../node-hub/dora-asr-mlx/target/release/dora-asr-mlx
  inputs:
    audio: mic-input/audio_segment
  outputs:
    - transcription
    - language_detected
    - processing_time
    - log
  env:
    PARAFORMER_MODEL_DIR: ~/.mofa/models/paraformer-large-mlx
    LOG_LEVEL: INFO
```

### Inputs

| Input | Type | Description |
|-------|------|-------------|
| `audio` | PyArrow Array | PCM audio (16kHz mono float32) |
| `control` | String | Commands: `stats`, `cleanup`, `reset` |

### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transcription` | String | Transcribed Chinese text |
| `language_detected` | String | Always "zh" |
| `processing_time` | Float | Inference duration (seconds) |
| `log` | JSON | Debug/status messages |

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PARAFORMER_MODEL_DIR` | `~/.mofa/models/paraformer-large-mlx` | Model directory path |
| `MIN_AUDIO_DURATION` | `0.5` | Minimum audio duration (seconds) |
| `MAX_AUDIO_DURATION` | `30.0` | Maximum audio duration (seconds) |
| `ASR_MLX_WARMUP` | `true` | Pre-initialize model on startup |
| `LOG_LEVEL` | `INFO` | Logging level |

## Performance

Benchmarks on Apple M3 Max:

| Audio Duration | Inference Time | RTF | Speed |
|----------------|----------------|-----|-------|
| 3s | ~50ms | 0.017 | 59x real-time |
| 10s | ~150ms | 0.015 | 67x real-time |
| 30s | ~400ms | 0.013 | 75x real-time |

## Comparison with dora-asr (Python)

| Aspect | dora-asr-mlx | dora-asr (Python) |
|--------|--------------|-------------------|
| Language | Rust | Python |
| Backend | MLX (Metal) | ONNX/PyTorch |
| Platform | Apple Silicon only | Cross-platform |
| Speed | 50-75x real-time | 5-10x real-time |
| Memory | ~1GB | ~2GB |
| Languages | Chinese only | Chinese + English (with Whisper) |

## Phase 2 Roadmap (Streaming)

Future versions will support streaming transcription:
- Chunked audio processing
- Partial results
- Lower latency for real-time applications

## License

Apache-2.0
