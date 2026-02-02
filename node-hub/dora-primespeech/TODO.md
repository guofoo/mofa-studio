
## Number Normalization Issue

**Date**: 2025-01-31

**Problem**: Arabic numerals like "30" are pronounced as individual digits "三零" (san ling) instead of the proper Chinese number "三十" (san shi).

**Root Cause**: The G2P (grapheme-to-phoneme) text preprocessing doesn't properly normalize Arabic numerals to Chinese number words.

**Affected Files**: Likely in `moyoyo_tts/text/chinese2.py` or text normalization module.

**Workaround**: Pre-normalize text before TTS:
```python
# Simple replacements
text = text.replace("30", "三十").replace("40", "四十")

# Or use a proper number-to-Chinese converter library
```

**Proper Fix**: Implement Chinese number normalization in the text preprocessing pipeline that converts:
- "30" → "三十"
- "2021" → "二零二一" or "两千零二十一" (context dependent)
- "30年" → "三十年"
- etc.

**Priority**: Medium - affects readability of content with numbers
