#!/usr/bin/env python3
"""
Rigorous benchmark for GPT-SoVITS few-shot TTS with multiple iterations.
"""

import sys
import time
import os
from pathlib import Path
import statistics

# Same text as Rust benchmark (without numbers)
TEST_TEXT = "我们说中国式现代化是百年大战略，这又分为三个阶段。第一个阶段，我们先用三十年时间建成了独立完整的工业体系和国民经济体系；再用四十年，全面建成了小康社会。我们现在正处于第三个阶段，这又被分成上下两篇：上半篇是基本实现社会主义现代化；下半篇是到本世纪中叶，建成社会主义现代化强国。"

NUM_ITERATIONS = 5

def main():
    print("=" * 80)
    print("RIGOROUS GPT-SoVITS Benchmark (Python + PyTorch/MPS)")
    print("=" * 80)
    print(f"Text: {len(TEST_TEXT)} characters")
    print(f"Iterations: {NUM_ITERATIONS}")
    print()

    # Setup
    primespeech_path = Path(__file__).parent / "../../../node-hub/dora-primespeech"
    primespeech_path = primespeech_path.resolve()
    model_dir = Path(os.path.expanduser("~/.dora/models/primespeech"))
    sys.path.insert(0, str(primespeech_path))
    os.environ['PRIMESPEECH_MODEL_DIR'] = str(model_dir)

    import torch
    device = 'mps' if torch.backends.mps.is_available() else 'cpu'
    print(f"Device: {device}")

    from dora_primespeech.moyoyo_tts_wrapper_streaming_fix import StreamingMoYoYoTTSWrapper

    # Initialize
    print("\nInitializing...")
    wrapper = StreamingMoYoYoTTSWrapper(voice='luoxiang', device=device, enable_streaming=False)

    # Warm-up (2 runs)
    print("Warm-up runs...")
    for i in range(2):
        _ = wrapper.synthesize("预热测试。", language='zh', speed=1.0)
    print("Warm-up complete.\n")

    # Benchmark runs
    times = []
    durations = []

    for i in range(NUM_ITERATIONS):
        start = time.time()
        sample_rate, audio_data = wrapper.synthesize(TEST_TEXT, language='zh', speed=1.0)
        elapsed = time.time() - start
        audio_duration = len(audio_data) / sample_rate

        times.append(elapsed)
        durations.append(audio_duration)
        rtf = audio_duration / elapsed
        print(f"  Run {i+1}: {elapsed:.2f}s synthesis, {audio_duration:.2f}s audio, {rtf:.2f}x RTF")

    print()
    print("=" * 80)
    print("PYTHON RESULTS")
    print("=" * 80)
    print(f"Synthesis time: min={min(times):.2f}s, max={max(times):.2f}s, avg={statistics.mean(times):.2f}s, stdev={statistics.stdev(times):.2f}s")
    print(f"Audio duration: avg={statistics.mean(durations):.2f}s")
    print(f"Real-time factor: avg={statistics.mean(durations)/statistics.mean(times):.2f}x")
    print(f"Chars/second: {len(TEST_TEXT)/statistics.mean(times):.1f}")
    print("=" * 80)

if __name__ == "__main__":
    main()
