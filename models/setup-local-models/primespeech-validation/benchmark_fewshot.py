#!/usr/bin/env python3
"""
Benchmark GPT-SoVITS few-shot TTS performance with dora-primespeech.
Uses the same text as the Rust benchmark for fair comparison.
"""

import sys
import time
import os
from pathlib import Path

# Similar Chinese text as Rust benchmark, with numbers converted to Chinese
TEST_TEXT = "我们说中国式现代化是百年大战略，这又分为三个阶段。第一个阶段，我们先用三十年时间建成了独立完整的工业体系和国民经济体系；再用四十年，全面建成了小康社会。我们现在正处于第三个阶段，这又被分成上下两篇：上半篇是基本实现社会主义现代化；下半篇是到本世纪中叶，建成社会主义现代化强国。"

def main():
    print("=" * 80)
    print("GPT-SoVITS Few-shot TTS Performance Benchmark (dora-primespeech / Python + PyTorch)")
    print("=" * 80)
    print()
    print(f"Voice: luoxiang")
    print(f"Text length: {len(TEST_TEXT.encode('utf-8'))} bytes, {len(TEST_TEXT)} characters")
    print("-" * 60)
    print(TEST_TEXT)
    print("-" * 60)
    print()

    # Setup paths
    primespeech_path = Path(__file__).parent / "../../../node-hub/dora-primespeech"
    primespeech_path = primespeech_path.resolve()
    model_dir = Path(os.path.expanduser("~/.dora/models/primespeech"))

    # Add PrimeSpeech to path
    sys.path.insert(0, str(primespeech_path))

    # Set environment variable
    os.environ['PRIMESPEECH_MODEL_DIR'] = str(model_dir)

    print(f"Using PrimeSpeech path: {primespeech_path}")
    print(f"Using model directory: {model_dir}")
    print()

    # Auto-detect best device
    import torch
    if torch.cuda.is_available():
        device = 'cuda'
    elif torch.backends.mps.is_available():
        device = 'mps'
    else:
        device = 'cpu'
    print(f"Device: {device}")

    # Import TTS wrapper
    from dora_primespeech.moyoyo_tts_wrapper_streaming_fix import StreamingMoYoYoTTSWrapper

    # Initialize TTS
    print()
    print("Initializing TTS engine...")
    init_start = time.time()

    wrapper = StreamingMoYoYoTTSWrapper(
        voice='luoxiang',
        device=device,
        enable_streaming=False
    )

    init_time = time.time() - init_start
    print(f"Initialization time: {init_time:.2f}s")

    # Warm-up run
    print()
    print("Warm-up run...")
    warmup_text = "你好，这是预热测试。"
    _ = wrapper.synthesize(warmup_text, language='zh', speed=1.0)
    print("Warm-up complete.")

    # Generate audio (main benchmark)
    print()
    print("Generating audio (benchmark run)...")
    synthesis_start = time.time()

    sample_rate, audio_data = wrapper.synthesize(TEST_TEXT, language='zh', speed=1.0)

    synthesis_time = time.time() - synthesis_start
    audio_duration = len(audio_data) / sample_rate

    # Calculate metrics
    chars_per_second = len(TEST_TEXT) / synthesis_time
    real_time_factor = audio_duration / synthesis_time

    # Save audio
    import soundfile as sf
    output_path = "/tmp/benchmark_python_fewshot.wav"
    sf.write(output_path, audio_data, sample_rate)

    # Print results
    print()
    print("=" * 80)
    print("RESULTS (dora-primespeech / Python + PyTorch)")
    print("=" * 80)
    print(f"Text length: {len(TEST_TEXT)} characters")
    print(f"Audio duration: {audio_duration:.2f} seconds")
    print(f"Synthesis time: {synthesis_time:.2f} seconds")
    print(f"Real-time factor: {real_time_factor:.2f}x {'(faster than real-time)' if real_time_factor > 1 else '(slower than real-time)'}")
    print(f"Processing speed: {chars_per_second:.1f} characters/second")
    print()
    print(f"Audio saved to: {output_path}")
    print("=" * 80)


if __name__ == "__main__":
    main()
