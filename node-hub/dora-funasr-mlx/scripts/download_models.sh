#!/bin/bash
# Download Paraformer-large MLX model for dora-asr-mlx
#
# Requirements:
#   - huggingface-cli installed (pip install huggingface_hub)
#
# Usage:
#   ./download_models.sh

set -e

MODEL_DIR="${PARAFORMER_MODEL_DIR:-$HOME/.mofa/models/paraformer-large-mlx}"

echo "Downloading Paraformer-large MLX model to: $MODEL_DIR"

# Create directory
mkdir -p "$MODEL_DIR"

# Download from HuggingFace
huggingface-cli download funaudiollm/paraformer-large-mlx \
    --local-dir "$MODEL_DIR" \
    --local-dir-use-symlinks False

echo ""
echo "Model downloaded successfully!"
echo ""
echo "Expected files:"
echo "  - $MODEL_DIR/paraformer.safetensors"
echo "  - $MODEL_DIR/am.mvn"
echo "  - $MODEL_DIR/tokens.txt"
echo ""

# Verify files exist
if [ -f "$MODEL_DIR/paraformer.safetensors" ] && \
   [ -f "$MODEL_DIR/am.mvn" ] && \
   [ -f "$MODEL_DIR/tokens.txt" ]; then
    echo "All required files present."
else
    echo "ERROR: Some required files are missing!"
    ls -la "$MODEL_DIR"
    exit 1
fi
