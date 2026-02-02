#!/usr/bin/env python3
"""
Audio sink that receives synthesized audio and saves to WAV files.
Produces individual WAVs, a combined WAV, and a summary report.
"""

import os
import time
import struct
import wave
import numpy as np
import pyarrow as pa
from dora import Node


OUTPUT_DIR = "/tmp/gpt-sovits-mlx-test"


def write_wav(path, samples, sr=32000):
    """Write float32 samples to WAV file."""
    with wave.open(path, 'w') as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(sr)
        int_samples = [max(-32768, min(32767, int(s * 32768))) for s in samples]
        data = struct.pack(f'<{len(int_samples)}h', *int_samples)
        wf.writeframes(data)


def main():
    node = Node()

    os.makedirs(OUTPUT_DIR, exist_ok=True)
    audio_count = 0
    results = []  # (question_id, duration, sample_count, recv_time)
    all_samples = []
    sample_rate = 32000
    start_time = time.time()

    print(f"[audio-sink] Saving audio to {OUTPUT_DIR}")

    for event in node:
        if event["type"] == "INPUT":
            input_id = event["id"]
            metadata = event.get("metadata", {})

            if input_id == "audio":
                recv_time = time.time() - start_time

                # Extract audio samples
                audio_data = event["value"]
                if isinstance(audio_data, (pa.Array, pa.ChunkedArray)):
                    samples = audio_data.to_numpy().astype(np.float32)
                else:
                    samples = np.array(audio_data, dtype=np.float32)

                # Get metadata
                question_id = metadata.get("question_id", "unknown")
                sr = int(metadata.get("sample_rate", 32000))
                sample_rate = sr
                duration = len(samples) / sr
                is_final = metadata.get("is_final", "false")
                frag_idx = metadata.get("fragment_index", "0")

                audio_count += 1
                output_path = os.path.join(OUTPUT_DIR, f"audio_{audio_count:03d}_q{question_id}.wav")

                # Save individual WAV
                write_wav(output_path, samples.tolist(), sr)

                results.append((question_id, duration, len(samples), recv_time))

                # Accumulate for combined output
                all_samples.extend(samples.tolist())
                # Add 0.3s silence between segments
                all_samples.extend([0.0] * int(sr * 0.3))

                print(f"[audio-sink] #{audio_count} q={question_id} frag={frag_idx} "
                      f"dur={duration:.2f}s samples={len(samples)} final={is_final} "
                      f"-> {os.path.basename(output_path)}")

            elif input_id == "segment_complete":
                status = event["value"][0].as_py() if hasattr(event["value"][0], "as_py") else str(event["value"][0])
                question_id = metadata.get("question_id", "unknown")
                session_status = metadata.get("session_status", "unknown")
                print(f"[audio-sink] Segment complete: q={question_id} status={status} session={session_status}")

                if status == "error":
                    error_msg = metadata.get("error", "unknown error")
                    print(f"[audio-sink] ERROR: {error_msg}")

        elif event["type"] == "STOP":
            break

    # Save combined audio
    if all_samples:
        combined_path = os.path.join(OUTPUT_DIR, "all_combined.wav")
        write_wav(combined_path, all_samples, sample_rate)
        combined_duration = len(all_samples) / sample_rate
        print(f"\n[audio-sink] Combined: {combined_path} ({combined_duration:.2f}s)")

    # Print summary
    total_duration = sum(r[1] for r in results)
    total_time = time.time() - start_time

    print(f"\n{'=' * 65}")
    print(f"  TTS Interface Test Summary")
    print(f"{'=' * 65}")
    print(f"{'#':<4} {'QID':>4} {'Duration':>9} {'Samples':>9}  {'Recv @':>8}")
    print(f"{'-'*4} {'-'*4} {'-'*9} {'-'*9}  {'-'*8}")
    for i, (qid, dur, n_samples, recv_t) in enumerate(results):
        print(f"{i+1:<4} {qid:>4} {dur:>8.2f}s {n_samples:>9}  {recv_t:>7.1f}s")
    print(f"{'-'*4} {'-'*4} {'-'*9} {'-'*9}  {'-'*8}")
    print(f"Total audio: {total_duration:.2f}s  |  Wall time: {total_time:.1f}s  |  Files: {audio_count}")
    print(f"\nOutput: {OUTPUT_DIR}")
    if all_samples:
        print(f"Play combined: afplay {os.path.join(OUTPUT_DIR, 'all_combined.wav')}")
    print(f"{'=' * 65}")


if __name__ == "__main__":
    main()
