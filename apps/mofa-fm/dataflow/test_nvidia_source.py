#!/usr/bin/env python3
"""Simple test source that sends a prompt to NVIDIA LLM"""

import time
import pyarrow as pa
from dora import Node

def main():
    node = Node()

    # Wait a moment for other nodes to initialize
    time.sleep(2)

    # Send test prompt as Arrow string array (what maas-client expects)
    prompt = "What is 2+2? Answer in one word only."

    print(f"Sending prompt: {prompt}")

    # Create Arrow array with the text
    arrow_array = pa.array([prompt])
    node.send_output("text", arrow_array)

    # Wait for response by listening to events
    print("Waiting for response...")
    for event in node:
        if event["type"] == "INPUT":
            input_id = event["id"]
            value = event["value"]
            # Try to decode as string array
            try:
                if hasattr(value, 'to_pylist'):
                    data = value.to_pylist()
                    if data and isinstance(data[0], str):
                        text = " ".join(data)
                    else:
                        text = bytes(data).decode('utf-8')
                else:
                    text = str(value)
                print(f"[{input_id}] Response: {text}")
            except Exception as e:
                print(f"[{input_id}] Raw: {value}, Error: {e}")
        elif event["type"] == "STOP":
            print("Done!")
            break

if __name__ == "__main__":
    main()
