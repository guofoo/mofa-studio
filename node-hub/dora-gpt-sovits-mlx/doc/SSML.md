# SSML Support for dora-gpt-sovits-mlx

SSML (Speech Synthesis Markup Language) enables fine-grained control over TTS synthesis including pauses, speed changes, and text segmentation.

## Auto-Detection

SSML is automatically detected when input text starts with `<speak>`. No configuration needed. Plain text input continues to work unchanged.

## Supported Tags

| Tag | Description | Example |
|-----|-------------|---------|
| `<speak>` | Root element (required) | `<speak>Hello</speak>` |
| `<break time="..."/>` | Insert silence (ms or s) | `<break time="500ms"/>` |
| `<break strength="..."/>` | Insert named pause | `<break strength="strong"/>` |
| `<prosody rate="...">` | Control speech speed | `<prosody rate="fast">text</prosody>` |
| `<s>` | Sentence boundary | `<s>A sentence.</s>` |
| `<p>` | Paragraph boundary (750ms pause) | `<p>A paragraph.</p>` |

## Prosody Rate Values

| Keyword | Speed Multiplier |
|---------|-----------------|
| `x-slow` | 0.5x |
| `slow` | 0.75x |
| `medium` | 1.0x (default) |
| `fast` | 1.25x |
| `x-fast` | 1.75x |
| Custom | `80%`, `120%`, etc. |

## Break Strength Values

| Strength | Duration |
|----------|----------|
| `none` | 0ms |
| `x-weak` | 100ms |
| `weak` | 200ms |
| `medium` | 400ms (default) |
| `strong` | 750ms |
| `x-strong` | 1200ms |

## Complete Example

```xml
<speak>
  Welcome to the weather report.
  <break time="500ms"/>

  <prosody rate="fast">
    Today's temperature is 25 degrees with clear skies.
  </prosody>

  <break strength="strong"/>

  <p>
    Tomorrow will be cloudy with a chance of rain.
  </p>
  <p>
    Please carry an umbrella just in case.
  </p>

  <prosody rate="slow">
    Thank you for listening.
  </prosody>
</speak>
```

This produces:
1. "Welcome to the weather report." at normal speed
2. 500ms silence
3. Weather details at 1.25x speed
4. 750ms silence
5. Two paragraphs at normal speed with 750ms pause between them
6. Closing at 0.75x speed

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RETURN_FRAGMENT` | `false` | Enable streaming mode (bypassed when SSML detected) |
| `FRAGMENT_INTERVAL` | `0.3` | Silence between fragments in seconds (plain text mode only) |
| `FRAGMENT_MIN_CHARS` | `10` | Minimum chars per fragment (plain text mode only) |

Note: When SSML is detected, `RETURN_FRAGMENT` and `FRAGMENT_INTERVAL` settings are ignored. SSML defines its own segmentation and pauses via `<break>` tags.

## Limitations

The following SSML tags are **not supported** (text content is preserved, tag semantics ignored):

- `<phoneme>` - Custom pronunciation (requires G2P bypass)
- `<emphasis>` - Stress/emphasis (not supported by GPT-SoVITS model)
- `<voice>` - Voice switching mid-speech (requires multi-reference loading)
- `<pitch>` - Pitch control (not in GPT-SoVITS)
- `<say-as>` - Number/date interpretation (partially handled by text normalizer)
- `<sub>` - Substitution aliases

Silence durations are capped at 10 seconds maximum.
