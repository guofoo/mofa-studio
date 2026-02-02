# MoFA Studio Critical Code Review (ASR Crash)

Date: 2026-01-27 (updated)
Scope: ASR pipeline + Dora bridge + UI integration

## Executive Summary
MoFA ASR crashed on every run due to `BotId::default()` producing a malformed empty string that panics inside moly-kit's `draw_list`. Secondary risks remain around dual audio capture and FFI safety.

## Root Cause (RESOLVED)
- `BotId::default()` creates an empty `Arc<str>("")`, but moly-kit's `Messages::draw_list` calls `BotId::id()` → `deconstruct()` which expects the format `<len>;<id>@<provider>`. The empty string has no `;`, so `split_once(';').expect("malformed bot id")` panics. This panic occurs inside a macOS `extern "C"` timer callback (`received_timer`), causing `panic_cannot_unwind` abort.
  - `aitk/src/protocol/entity.rs:143`
  - `apps/mofa-asr/src/screen/mod.rs:299` (call site)
- **Fix:** `BotId::default()` → `BotId::new("asr", "local")`

## Previously Suspected (NOT the crash cause)
- Native AEC init race and FFI buffer handling were suspected but not involved. The crash occurred in the UI draw path, not the audio path.
  - `mofa-dora-bridge/src/widgets/aec_input.rs:150-238`

## High-Risk Findings
- Dual capture: UI mic monitoring (CPAL) starts unconditionally while the AEC bridge also starts recording on dataflow start. Two concurrent input streams on macOS can cause stream failures or crashes once audio becomes active.
  - `apps/mofa-asr/src/screen/mod.rs:428-434`
  - `apps/mofa-asr/src/screen/mod.rs:483-514`
  - `mofa-dora-bridge/src/widgets/aec_input.rs:520-620`
- Paraformer node duration math uses a fixed 16kHz for `audio_duration` in `process_audio`, ignoring the actual `sample_rate` from metadata. With 48kHz input, duration and min/max gating are wrong and can create edge cases.
  - `node-hub/dora-funasr-mlx/src/main.rs:208-237`

## Medium-Risk Findings
- Process-global env mutation inside the worker thread is not scoped; values persist and can leak across sessions or components.
  - `apps/mofa-asr/src/dora_integration.rs:200-210`
- UI crash risk from mutex poisoning: `expect("ChatController mutex poisoned")` will hard panic on any previous panic while holding the lock, turning transient errors into fatal UI crashes.
  - `apps/mofa-asr/src/screen/mod.rs:327-353`
- FFI symbol lifetime is extended via `transmute` without a safety proof; if the worker thread calls into dropped symbols, behavior is undefined.
  - `mofa-dora-bridge/src/widgets/aec_input.rs:105-136`

## Reproduction / Isolation Steps
1. Disable AEC before start and force CPAL-only path. If the crash disappears, the native AEC library is implicated.
2. Disable UI mic monitoring when Dora AEC capture is active to avoid dual streams.
3. Collect macOS crash report/backtrace to confirm faulting thread (AudioUnit vs Rust).

## Suggested Next Actions
- Guard `NativeAudioCapture::start()` to only set `init_successful = true` after confirmed audio data OR an explicit native “init complete” callback.
- Add defensive checks in `get_audio()` for size alignment and maximum expected buffer length.
- Serialize mic capture: either UI mic monitoring OR AEC capture, not both.
- Fix `audio_duration` calculation to use the metadata `sample_rate` consistently.
