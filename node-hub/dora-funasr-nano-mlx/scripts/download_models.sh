#!/bin/bash
# Download Fun-ASR-Nano model for dora-asr-nano-mlx
#
# Requirements:
#   - huggingface-cli installed (pip install huggingface_hub)
#
# Usage:
#   ./download_models.sh [base|multilingual]
#
# Options:
#   base (default) - Chinese/English/Japanese (3 languages)
#   multilingual   - 31 languages support

set -e

MODEL_VARIANT="${1:-base}"
MODEL_DIR="${SENSEVOICE_MODEL_DIR:-$HOME/.mofa/models/Fun-ASR-Nano-2512}"

echo "Downloading Fun-ASR-Nano model ($MODEL_VARIANT) to: $MODEL_DIR"

# Create directory
mkdir -p "$MODEL_DIR"

# Select model based on variant
if [ "$MODEL_VARIANT" == "multilingual" ]; then
    MODEL_REPO="funaudiollm/Fun-ASR-MLT-Nano-2512"
    echo "Using multilingual model (31 languages)"
else
    MODEL_REPO="funaudiollm/Fun-ASR-Nano-2512"
    echo "Using base model (Chinese/English/Japanese)"
fi

# Download from HuggingFace
huggingface-cli download "$MODEL_REPO" \
    --local-dir "$MODEL_DIR" \
    --local-dir-use-symlinks False

echo ""
echo "Model downloaded successfully!"
echo ""
echo "Expected files:"
echo "  - $MODEL_DIR/config.yaml"
echo "  - $MODEL_DIR/model.safetensors"
echo "  - $MODEL_DIR/Qwen3-0.6B/tokenizer.json"
echo ""

# Verify files exist
if [ -f "$MODEL_DIR/config.yaml" ] && \
   [ -f "$MODEL_DIR/model.safetensors" ] && \
   [ -d "$MODEL_DIR/Qwen3-0.6B" ]; then
    echo "All required files present."
else
    echo "ERROR: Some required files are missing!"
    ls -la "$MODEL_DIR"
    exit 1
fi
