#!/usr/bin/env python3
"""Compare phonemes output from Python GPT-SoVITS"""
import os
import sys

# Get the directory of this script
script_dir = os.path.dirname(os.path.abspath(__file__))
os.chdir(script_dir)
sys.path.insert(0, script_dir)

from moyoyo_tts.text import cleaned_text_to_sequence
from moyoyo_tts.text.cleaner import clean_text

text = "从季节上看，主要是增在秋粮，2025年秋粮增产163.6亿斤，占全年粮食增量九成多。从区域上看，主要增在东北三省、内蒙古和新疆，这5个省区粮食增产114.7亿斤，占全国增量接近70%。从品种上看，主要增在玉米，玉米增产126.4亿斤，占全年粮食增量的75%。农业农村部副部长张兴旺分析说。"

# Clean text (this does normalization + g2p)
phones, word2ph, norm_text = clean_text(text, "zh")

print(f"Python normalized text: {norm_text}")
print(f"\nPython phoneme count: {len(phones)}")
print("\nPython phonemes:")
for i, p in enumerate(phones):
    print(f"{i}:{p}", end=", ")
    if (i + 1) % 20 == 0:
        print()
print()

# Also get the IDs
phone_ids = cleaned_text_to_sequence(phones, "v2")
print(f"\nPython phone IDs count: {len(phone_ids)}")
print("Python phone IDs:")
for i, pid in enumerate(phone_ids):
    print(f"{i}:{pid}", end=", ")
    if (i + 1) % 20 == 0:
        print()
print()
