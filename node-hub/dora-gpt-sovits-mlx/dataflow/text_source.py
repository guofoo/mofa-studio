#!/usr/bin/env python3
"""
Text source that sends test sentences for TTS synthesis.
Waits for tick events to pace the sending.
"""

import pyarrow as pa
from dora import Node

# Test sentences (mix of Chinese and English)
TEST_SENTENCES = [
    "你好，世界！这是一个测试。",
    "今天天气真好，我们去公园走走吧。",
    "Hello world, this is a test sentence.",
    "人工智能正在改变我们的生活方式。",
    "这是最后一句测试语音。",
]


def main():
    node = Node()

    current_idx = 0
    tick_count = 0
    # Wait a few ticks before sending first sentence (let TTS initialize)
    wait_ticks = 10  # 5 seconds at 500ms per tick
    # Wait between sentences
    interval_ticks = 20  # 10 seconds between sentences
    next_send_tick = wait_ticks

    print(f"[text-source] Ready to send {len(TEST_SENTENCES)} test sentences")

    for event in node:
        if event["type"] == "INPUT" and event["id"] == "tick":
            tick_count += 1

            if current_idx < len(TEST_SENTENCES) and tick_count >= next_send_tick:
                text = TEST_SENTENCES[current_idx]
                question_id = str(current_idx + 1)

                metadata = {
                    "question_id": question_id,
                    "session_status": "active",
                    "segment_index": str(current_idx),
                }

                text_array = pa.array([text])
                print(f"[text-source] Sending [{current_idx + 1}/{len(TEST_SENTENCES)}]: {text}")
                node.send_output("text", text_array, metadata)

                current_idx += 1
                next_send_tick = tick_count + interval_ticks

            # Exit after all sentences sent + final wait
            if current_idx >= len(TEST_SENTENCES) and tick_count >= next_send_tick:
                print(f"[text-source] All {len(TEST_SENTENCES)} sentences sent. Exiting.")
                break

        elif event["type"] == "STOP":
            print("[text-source] Stopping...")
            break


if __name__ == "__main__":
    main()
