#!/usr/bin/env python3
"""Compare phonemes output from Python GPT-SoVITS - direct import bypassing __init__"""
import os
import sys

# Set up paths
script_dir = os.path.dirname(os.path.abspath(__file__))
text_dir = script_dir  # we're in moyoyo_tts/text
moyoyo_dir = os.path.dirname(script_dir)
parent_dir = os.path.dirname(moyoyo_dir)

# Block moyoyo_tts from loading by creating an empty mock
import types
sys.modules["moyoyo_tts"] = types.ModuleType("moyoyo_tts")
sys.modules["moyoyo_tts"].__path__ = [moyoyo_dir]

# Create moyoyo_tts.text mock
sys.modules["moyoyo_tts.text"] = types.ModuleType("moyoyo_tts.text")
sys.modules["moyoyo_tts.text"].__path__ = [text_dir]

# Now add parent to path and import what we need
sys.path.insert(0, parent_dir)

# Execute symbols2.py directly
symbols2_dict = {}
exec(open(os.path.join(text_dir, "symbols2.py")).read(), symbols2_dict)
symbols_v2 = symbols2_dict["symbols"]
symbol_to_id_v2 = {s: i for i, s in enumerate(symbols_v2)}

# Execute symbols.py for punctuation
symbols_dict = {}
exec(open(os.path.join(text_dir, "symbols.py")).read(), symbols_dict)
punctuation = symbols_dict["punctuation"]

# Mock moyoyo_tts.text.symbols
symbols_mock = types.ModuleType("moyoyo_tts.text.symbols")
symbols_mock.punctuation = punctuation
symbols_mock.symbols = symbols_dict["symbols"]
sys.modules["moyoyo_tts.text.symbols"] = symbols_mock

# Import tone_sandhi
from moyoyo_tts.text import tone_sandhi
sys.modules["moyoyo_tts.text.tone_sandhi"] = tone_sandhi

# Import chinese (for text_normalize and g2p)
from moyoyo_tts.text import chinese as chinese_mod
sys.modules["moyoyo_tts.text.chinese"] = chinese_mod

text = "从季节上看，主要是增在秋粮，2025年秋粮增产163.6亿斤，占全年粮食增量九成多。从区域上看，主要增在东北三省、内蒙古和新疆，这5个省区粮食增产114.7亿斤，占全国增量接近70%。从品种上看，主要增在玉米，玉米增产126.4亿斤，占全年粮食增量的75%。农业农村部副部长张兴旺分析说。"

# Normalize text
norm_text = chinese_mod.text_normalize(text)
print(f"Python normalized text: {norm_text}")

# G2P
phones, word2ph = chinese_mod.g2p(norm_text)

print(f"\nPython phoneme count: {len(phones)}")
print("\nPython phonemes:")
for i, p in enumerate(phones):
    print(f"{i}:{p}", end=", ")
    if (i + 1) % 20 == 0:
        print()
print()

# Get the IDs
phone_ids = [symbol_to_id_v2[p] for p in phones if p in symbol_to_id_v2]
print(f"\nPython phone IDs count: {len(phone_ids)}")
print("Python phone IDs:")
for i, pid in enumerate(phone_ids):
    print(f"{i}:{pid}", end=", ")
    if (i + 1) % 20 == 0:
        print()
print()
