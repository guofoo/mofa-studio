# MoFA ASR Development Plan

## MANDATORY: Use mofa-ui Shared Components

> **NEVER copy widget code from mofa-fm or other apps.**
> Always use shared widgets from `mofa-ui/src/widgets/`. These provide
> reusable, dark-mode-aware components with consistent APIs.
>
> **Required shared widgets:**
> - `<ChatPanel>` — chat/transcription display (`mofa_ui::widgets::chat_panel`)
> - `<MofaLogPanel>` — filterable system log (`mofa_ui::widgets::log_panel`)
> - `<MicButton>` — mic toggle (`mofa_ui::widgets::mic_button`)
> - `<AecButton>` — AEC toggle (`mofa_ui::widgets::aec_button`)
> - `<LedMeter>` — audio level meter (`mofa_ui::widgets::led_meter`)
> - `<MofaHero>` — start/stop + system stats (`mofa_ui::widgets::mofa_hero`)
>
> **Import pattern in `live_design!`:**
> ```
> use mofa_ui::widgets::chat_panel::ChatPanel;
> use mofa_ui::widgets::log_panel::MofaLogPanel;
> ```
>
> **Register in `lib.rs`:**
> ```rust
> mofa_ui::widgets::chat_panel::live_design(cx);
> mofa_ui::widgets::log_panel::live_design(cx);
> ```
>
> **Use Ref APIs in `mod.rs`:**
> ```rust
> self.view.chat_panel(ids!(chat_panel)).set_messages(cx, &messages);
> self.view.mofa_log_panel(ids!(log_panel)).add_log(cx, &formatted);
> ```

## Priority Legend
- **P0.x**: Critical path - app won't run without these
- **P1.x**: Core features - basic functionality
- **P2.x**: Enhanced features - settings and configuration
- **P3.x**: Polish - integration and refinement

---

## P0: Project Setup (Critical Path)

### P0.0 - Directory Structure
- [x] Create `apps/mofa-asr/` directory
- [x] Create `apps/mofa-asr/src/screen/` directory
- [x] Create `apps/mofa-asr/dataflow/` directory
- [x] Create `apps/mofa-asr/README.md`

### P0.1 - Cargo Configuration
- [x] Create `apps/mofa-asr/Cargo.toml`
- [x] Add `apps/mofa-asr` to workspace in root `Cargo.toml` (uses `apps/*` glob)

### P0.2 - App Registration
- [x] Create `apps/mofa-asr/src/lib.rs` (MofaApp trait)
- [x] Register app in `mofa-studio-shell/src/app.rs`
- [x] Add to `mofa-studio-shell/Cargo.toml` as dependency
- [x] Add ASR page to `dashboard.rs`
- [x] Add `PageId::Asr` to `mofa-widgets/src/app_trait.rs`
- [x] Add `mofa_asr_tab` to sidebar.rs (SidebarMenuButton, click handling, selection, dark mode)

### P0.3 - Basic Screen Widget
- [x] Create `apps/mofa-asr/src/screen/mod.rs` (minimal widget)
- [x] Create `apps/mofa-asr/src/screen/design.rs` (minimal UI)

### P0.4 - Build Verification
- [x] Run `cargo build -p mofa-asr`
- [x] Run `cargo build -p mofa-studio-shell`
- [ ] Verify app appears in shell sidebar and navigates correctly

---

## P1: Core Features

### P1.0 - Dora Integration
- [x] Create `apps/mofa-asr/src/dora_integration.rs`
- [x] Implement DoraCommand enum (Start, Stop, Recording, AEC)
- [x] Implement DoraEvent enum (Started, Stopped, Error)
- [x] Implement DoraIntegration struct with worker thread

### P1.1 - Hero Panel Integration (mofa-ui: MofaHero)
- [x] Add `<MofaHero>` to design.rs
- [x] Wire Start/Stop buttons to dora_integration
- [x] Handle connection status updates

### P1.2 - Audio Controls (mofa-ui: MicButton, AecButton, LedMeter)
- [x] Add `<MicButton>`, `<AecButton>`, `<LedMeter>` to UI (in design.rs)
- [x] Implement AudioManager initialization
- [x] Add device selection dropdowns (input/output)
- [x] Wire mic level updates to LedMeter (50ms timer)
- [x] Wire mic mute button toggle
- [x] Wire AEC button to dora integration

### P1.3 - Chat Window Output (mofa-ui: ChatPanel)
- [x] Use `<ChatPanel>` from mofa-ui (NOT custom Markdown widget)
- [x] Set `empty_text: "Waiting for speech input..."`
- [x] Use `ChatPanel::set_messages(cx, &messages)` API with `ChatMessage` structs
- [x] Copy via `ChatPanel::copy_clicked(actions)` + `get_text_for_copy()`
- [x] Auto-scroll handled by ChatPanel internally
- [x] Poll transcriptions from SharedDoraState.chat (via mofa-prompt-input → PromptInputBridge)
- [x] Data flow: asr/transcription → mofa-prompt-input (dynamic) → PromptInputBridge → SharedDoraState.chat → UI
- [x] Dark mode via `ChatPanel::apply_dark_mode(cx, dm)`

### P1.4 - Dataflow Configuration
- [x] Create `apps/mofa-asr/dataflow/asr-nano-mlx.yml` (SenseVoice, auto language detect)
- [x] Create `apps/mofa-asr/dataflow/asr-mlx.yml` (Paraformer, Chinese only)
- [x] Create `apps/mofa-asr/dataflow/asr-dual.yml` (Both models)
- [x] Add mofa-prompt-input to all dataflows (receives asr/transcription on human_text port)
- [x] Dynamic nodes: mofa-mic-input, mofa-prompt-input, mofa-system-log
- [ ] Test dataflow manually with `dora start`
- [ ] Verify transcription output end-to-end

### P1.5 - System Log Panel (mofa-ui: MofaLogPanel)
- [x] Use `<MofaLogPanel>` from mofa-ui (NOT custom log panel code)
- [x] Add to design.rs layout (inside transcription_tab_content)
- [x] Poll SharedDoraState.logs in poll_dora_events (format as `[LEVEL] [NODE] message`)
- [x] Use `MofaLogPanel::add_log(cx, &formatted)` + `update_if_dirty(cx)` APIs
- [x] Copy via `MofaLogPanel::copy_clicked(actions)` + `get_filtered_logs()`
- [x] Dark mode via `MofaLogPanel::apply_dark_mode(cx, dm)`
- [x] Filtering (level, node, search) handled by MofaLogPanel internally
- [ ] Poll Rust logs via log_bridge::poll_logs() on audio timer (50ms)

### P1.6 - Audio Panel (match mofa-fm exactly)
- [x] Use mofa-ui widgets: MicButton, AecButton, LedMeter (same imports as mofa-fm)
- [x] Match mofa-fm container structure: audio_container → audio_controls_row + device_container
- [ ] Verify audio panel renders identically to mofa-fm (visual test)
- [ ] Ensure all widget paths in mod.rs match the DSL layout

### P1.7 - Sidebar & Navigation
- [x] Add `SidebarSelection::Asr` to sidebar enum
- [x] Add `mofa_asr_tab` button to sidebar live_design
- [x] Wire click → selection → page navigation
- [x] Add to clear_all_selections, restore_selection_state
- [x] Add dark mode support for ASR tab in sidebar
- [x] Add dark mode for ASR screen in app.rs (apply_dark_mode_screens_with_value)

---

## P2: Settings & Configuration

### P2.0 - Tab System
- [x] Add Transcription/Settings tabs to design.rs
- [x] Implement tab switching logic in mod.rs
- [ ] Create `apps/mofa-asr/src/screen/settings.rs` (separate module)

### P2.1 - Model Selection
- [x] Add model selection labels in settings tab (Paraformer/SenseVoice/Both)
- [ ] Wire model radio buttons to AsrModelSelection
- [ ] Implement dataflow selection based on model choice

### P2.2 - SenseVoice Language Settings
- [x] Add language dropdown in settings tab (auto/zh/en/ja)
- [ ] Wire language dropdown to settings.sensevoice_language
- [ ] Pass ASR_NANO_LANGUAGE env var to dataflow (already wired in handle_start)

### P2.3 - Duration Settings
- [ ] Add min/max duration sliders
- [ ] Pass MIN_AUDIO_DURATION, MAX_AUDIO_DURATION env vars

### P2.4 - Advanced Settings
- [ ] Add warmup toggle checkbox
- [ ] Add custom dataflow picker (mofa-ui: DataflowPicker if applicable)
- [ ] Pass ASR_MLX_WARMUP env var

### P2.5 - Settings Persistence
- [ ] Add ASR fields to mofa-settings/preferences.rs
- [ ] Implement load/save in settings.rs

---

## P3: Polish & Integration

### P3.0 - Dark Mode Support
- [x] Add dark_mode instance vars to design.rs shaders
- [x] Implement update_dark_mode in mod.rs (calls apply_dark_mode on all mofa-ui widgets)
- [x] Wire dark mode in shell app.rs
- [ ] Test light/dark mode transitions visually

### P3.1 - Model Indicator
- [x] Add model indicator widget in audio controls bar (design.rs)
- [ ] Update indicator dynamically when model changes or dataflow starts

### P3.2 - Transcription Enhancements
- [ ] Show processing time (optional)
- [ ] Show language detection badge
- [ ] Distinguish models in dual mode (color/prefix)

### P3.3 - Error Handling
- [ ] Handle dataflow start failures gracefully
- [ ] Handle model loading errors
- [ ] Show user-friendly error messages in UI

### P3.4 - Testing & Documentation
- [ ] Test all three dataflow configurations
- [ ] Test settings persistence
- [ ] Update README with usage instructions

---

## Current Status

**Last Updated**: P1.3 and P1.5 refactored to use mofa-ui shared widgets (ChatPanel, MofaLogPanel)

**Current Task**: P1.6 Audio Panel verification → P2.0 settings wiring → P1.4 end-to-end test

**Completed**:
- P0.0 - Directory Structure
- P0.1 - Cargo Configuration
- P0.2 - App Registration (including sidebar button)
- P0.3 - Basic Screen Widget
- P0.4 - Build Verification (both packages build clean)
- P1.0 - Dora Integration
- P1.1 - Hero Panel Integration (mofa-ui: MofaHero)
- P1.2 - Audio Controls (mofa-ui: MicButton, AecButton, LedMeter)
- P1.3 - Chat Window Output (mofa-ui: ChatPanel) — refactored from custom Markdown
- P1.4 - Dataflow Configuration (3 YAMLs with mofa-prompt-input + mofa-system-log)
- P1.5 - System Log Panel (mofa-ui: MofaLogPanel) — uses shared widget, no copied code
- P1.7 - Sidebar & Navigation (button, selection, dark mode)
- P3.0 - Dark Mode Support (all mofa-ui widgets use apply_dark_mode)

**Next Steps**:
1. P1.6 - Verify audio panel renders identically to mofa-fm (visual test)
2. Verify app appears in shell sidebar (run `cargo run -p mofa-studio-shell`)
3. P2.0 - Wire settings tab interactions (model radio, language dropdown)
4. P1.4 - End-to-end dataflow test with `dora start`
5. P1.5 - Add Rust log polling via log_bridge::poll_logs()

**Key Design Decisions**:
- **mofa-ui is mandatory** — all UI widgets must come from mofa-ui shared components, never copied from other apps
- Chat output uses `mofa-prompt-input` dynamic node → `PromptInputBridge` → `SharedDoraState.chat` → `ChatPanel`
- System logs use `mofa-system-log` dynamic node → `SharedDoraState.logs` → `MofaLogPanel`
- ASR transcription arrives on `human_text` input port of `mofa-prompt-input`
- SenseVoice auto-detects language (zh/en/ja) post-transcription via Unicode character analysis
- `ASR_NANO_LANGUAGE` env var is loaded but currently unused by the model (auto mode is default)
- Register shared widget `live_design(cx)` in `lib.rs` before `screen::live_design(cx)`
