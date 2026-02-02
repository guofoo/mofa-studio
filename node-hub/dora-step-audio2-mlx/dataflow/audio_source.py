#!/usr/bin/env python3
"""
Audio file source that sends 10 individual segments one at a time.
Waits for tick events to pace the sending (one segment per tick after initial load).
"""

import os
import numpy as np
import pyarrow as pa
from dora import Node


def main():
    node = Node()

    audio_file = os.environ.get("TEST_AUDIO_FILE", "test.wav")
    segment_duration = float(os.environ.get("SEGMENT_DURATION", "4.0"))
    num_segments = int(os.environ.get("NUM_SEGMENTS", "10"))
    # Ticks to wait between segments (tick=100ms, so 50 = 5s wait for ASR processing)
    wait_ticks = int(os.environ.get("WAIT_TICKS", "50"))

    print(f"[audio-source] Loading audio file: {audio_file}")

    try:
        from scipy.io import wavfile
        sample_rate, audio_data = wavfile.read(audio_file)

        if audio_data.dtype == np.int16:
            audio_data = audio_data.astype(np.float32) / 32768.0
        elif audio_data.dtype == np.int32:
            audio_data = audio_data.astype(np.float32) / 2147483648.0
        elif audio_data.dtype != np.float32:
            audio_data = audio_data.astype(np.float32)

        if len(audio_data.shape) > 1:
            audio_data = audio_data.mean(axis=1)

        total_duration = len(audio_data) / sample_rate
        print(f"[audio-source] Loaded {len(audio_data)} samples at {sample_rate}Hz ({total_duration:.2f}s)")

    except Exception as e:
        print(f"[audio-source] ERROR: Failed to load audio file: {e}")
        return

    # Split into segments
    samples_per_segment = int(segment_duration * sample_rate)
    segments = []
    for i in range(num_segments):
        start = i * samples_per_segment
        end = start + samples_per_segment
        if start >= len(audio_data):
            break
        segment = audio_data[start:min(end, len(audio_data))]
        if len(segment) > sample_rate * 0.5:  # At least 0.5s
            segments.append(segment)

    print(f"[audio-source] Split into {len(segments)} segments of ~{segment_duration}s each")

    current_segment = 0
    tick_count = 0
    # Send first segment immediately on first tick, then wait between segments
    next_send_tick = 0

    for event in node:
        if event["type"] == "INPUT" and event["id"] == "tick":
            tick_count += 1

            if current_segment < len(segments) and tick_count >= next_send_tick:
                seg = segments[current_segment]
                seg_duration = len(seg) / sample_rate

                metadata = {
                    "sample_rate": str(sample_rate),
                    "question_id": str(current_segment + 1),
                    "segment": str(current_segment),
                }

                audio_array = pa.array(seg, type=pa.float32())
                print(f"[audio-source] Sending segment {current_segment + 1}/{len(segments)} ({seg_duration:.2f}s)")
                node.send_output("audio", audio_array, metadata)

                current_segment += 1
                next_send_tick = tick_count + wait_ticks

            # Exit after all segments sent + final wait
            if current_segment >= len(segments) and tick_count >= next_send_tick:
                print(f"[audio-source] All {len(segments)} segments sent. Exiting.")
                break

        elif event["type"] == "STOP":
            print("[audio-source] Stopping...")
            break


if __name__ == "__main__":
    main()
