#!/usr/bin/env python3
"""
Text source that sends test sentences for TTS synthesis.
Waits for tick events to pace the sending.
"""

import pyarrow as pa
from dora import Node

# Test sentences: 10 short texts (<25 words each), mix of Chinese and English
TEST_SENTENCES = [
    "你好，欢迎来到我们的节目。",
    "今天的天气非常好，阳光明媚。",
    "请问您需要什么帮助吗？",
    "这道菜的味道真的很不错。",
    "我们下周一开会讨论这个方案。",
    "The weather is beautiful today.",
    "Let me know if you have questions.",
    "科技改变了我们的生活方式。",
    "祝你生日快乐，万事如意！",
    "谢谢大家的支持和关注。",
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

    total = len(TEST_SENTENCES)
    print(f"[text-source] Ready to send {total} test sentences")

    for event in node:
        if event["type"] == "INPUT" and event["id"] == "tick":
            tick_count += 1

            if current_idx < total and tick_count >= next_send_tick:
                text = TEST_SENTENCES[current_idx]
                question_id = str(current_idx + 1)
                is_last = current_idx == total - 1

                metadata = {
                    "question_id": question_id,
                    "session_status": "ended" if is_last else "active",
                    "segment_index": str(current_idx),
                    "total_segments": str(total),
                }

                text_array = pa.array([text])
                print(f"[text-source] Sending [{current_idx + 1}/{total}]: {text}")
                node.send_output("text", text_array, metadata)

                current_idx += 1
                next_send_tick = tick_count + interval_ticks

            # Exit after all sentences sent + final wait
            if current_idx >= total and tick_count >= next_send_tick:
                print(f"[text-source] All {total} sentences sent. Exiting.")
                break

        elif event["type"] == "STOP":
            print("[text-source] Stopping...")
            break


if __name__ == "__main__":
    main()
