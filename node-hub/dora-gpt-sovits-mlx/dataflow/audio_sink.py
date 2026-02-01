#!/usr/bin/env python3
"""
Audio sink that receives synthesized audio and saves to WAV files.
"""

import os
import numpy as np
import pyarrow as pa
from dora import Node
from scipy.io import wavfile


OUTPUT_DIR = "/tmp/gpt-sovits-mlx-test"


def main():
    node = Node()

    os.makedirs(OUTPUT_DIR, exist_ok=True)
    audio_count = 0

    print(f"[audio-sink] Saving audio to {OUTPUT_DIR}")

    for event in node:
        if event["type"] == "INPUT":
            input_id = event["id"]
            metadata = event.get("metadata", {})

            if input_id == "audio":
                # Extract audio samples
                audio_data = event["value"]
                if isinstance(audio_data, pa.Array):
                    samples = audio_data.to_numpy()
                elif isinstance(audio_data, pa.ChunkedArray):
                    samples = audio_data.to_numpy()
                else:
                    samples = np.array(audio_data, dtype=np.float32)

                # Get metadata
                question_id = metadata.get("question_id", "unknown")
                sample_rate = int(metadata.get("sample_rate", 32000))
                duration = metadata.get("duration", "?")

                audio_count += 1
                output_path = os.path.join(OUTPUT_DIR, f"audio_{audio_count:03d}_q{question_id}.wav")

                # Convert to int16 for WAV
                samples_int16 = (samples * 32767).astype(np.int16)
                wavfile.write(output_path, sample_rate, samples_int16)

                print(f"[audio-sink] Saved: {output_path} ({duration}s, {len(samples)} samples)")

            elif input_id == "segment_complete":
                status = event["value"][0].as_py() if hasattr(event["value"][0], "as_py") else str(event["value"][0])
                question_id = metadata.get("question_id", "unknown")
                print(f"[audio-sink] Segment complete: status={status}, question_id={question_id}")

                if status == "error":
                    error_msg = metadata.get("error", "unknown error")
                    print(f"[audio-sink] ERROR: {error_msg}")

        elif event["type"] == "STOP":
            print(f"[audio-sink] Stopping. Total audio files saved: {audio_count}")
            break

    print(f"[audio-sink] Done. Audio files in: {OUTPUT_DIR}")


if __name__ == "__main__":
    main()
