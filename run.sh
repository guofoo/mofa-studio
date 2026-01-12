#!/usr/bin/env bash
# MoFA Studio 启动脚本
# 自动处理 Nix Flakes 实验性功能

set -euo pipefail

echo "[MoFA] Starting MoFA Studio..."
nix --extra-experimental-features 'nix-command flakes' run .
