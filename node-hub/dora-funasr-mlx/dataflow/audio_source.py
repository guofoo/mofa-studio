#!/usr/bin/env python3
"""
Simple audio file source node for testing ASR.

Reads a WAV file and sends audio chunks to the ASR node.
"""

import os
import numpy as np
import pyarrow as pa
from dora import Node

def main():
    node = Node()

    audio_file = os.environ.get("TEST_AUDIO_FILE", "test_audio.wav")
    sent = False

    print(f"[audio-source] Loading audio file: {audio_file}")

    # Load audio file
    try:
        from scipy.io import wavfile
        sample_rate, audio_data = wavfile.read(audio_file)

        # Convert to float32 normalized
        if audio_data.dtype == np.int16:
            audio_data = audio_data.astype(np.float32) / 32768.0
        elif audio_data.dtype == np.int32:
            audio_data = audio_data.astype(np.float32) / 2147483648.0
        elif audio_data.dtype != np.float32:
            audio_data = audio_data.astype(np.float32)

        # Convert stereo to mono if needed
        if len(audio_data.shape) > 1:
            audio_data = audio_data.mean(axis=1)

        print(f"[audio-source] Loaded {len(audio_data)} samples at {sample_rate}Hz ({len(audio_data)/sample_rate:.2f}s)")

    except Exception as e:
        print(f"[audio-source] ERROR: Failed to load audio file: {e}")
        print(f"[audio-source] Please set TEST_AUDIO_FILE environment variable to a valid WAV file")
        audio_data = None
        sample_rate = 16000

    for event in node:
        if event["type"] == "INPUT" and event["id"] == "tick":
            if audio_data is not None and not sent:
                # Send audio as PyArrow array
                audio_array = pa.array(audio_data, type=pa.float32())

                metadata = {
                    "sample_rate": str(sample_rate),
                    "question_id": "1",
                    "segment": "0"
                }

                print(f"[audio-source] Sending {len(audio_data)} samples...")
                node.send_output("audio", audio_array, metadata)
                sent = True
                print("[audio-source] Audio sent, waiting for transcription...")

        elif event["type"] == "STOP":
            print("[audio-source] Stopping...")
            break

if __name__ == "__main__":
    main()
