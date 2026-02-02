# MoFA Studio App Development Guide

> How to create apps for MoFA Studio using the MofaApp plugin system

---

## Overview

MoFA Studio uses a trait-based plugin system for apps. Each app:
1. Implements the `MofaApp` trait
2. Provides widgets via Makepad's `live_design!` macro
3. Registers with the shell at compile time

**Key Constraint**: Makepad requires compile-time widget type resolution. Apps cannot be loaded dynamically at runtime.

---

## Quick Start

### 1. Create App Crate

```bash
cd apps
cargo new mofa-myapp --lib
```

### 2. Configure Cargo.toml

```toml
[package]
name = "mofa-myapp"
version = "0.1.0"
edition = "2021"

[dependencies]
makepad-widgets = { workspace = true }
mofa-widgets = { path = "../../mofa-widgets" }
```

### 3. Implement MofaApp Trait

```rust
// src/lib.rs
pub mod screen;

use makepad_widgets::Cx;
use mofa_widgets::{MofaApp, AppInfo};

/// App descriptor - required for plugin system
pub struct MoFaMyApp;

impl MofaApp for MoFaMyApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "My App",           // Display name in UI
            id: "mofa-myapp",         // Unique identifier
            description: "My custom MoFA app",
        }
    }

    fn live_design(cx: &mut Cx) {
        screen::live_design(cx);
    }
}

/// Backwards-compatible registration function
pub fn live_design(cx: &mut Cx) {
    MoFaMyApp::live_design(cx);
}
```

### 4. Create Main Screen Widget

```rust
// src/screen.rs
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // Import shared theme (required)
    use mofa_widgets::theme::FONT_REGULAR;
    use mofa_widgets::theme::FONT_MEDIUM;
    use mofa_widgets::theme::DARK_BG;
    use mofa_widgets::theme::TEXT_PRIMARY;

    // Define your screen widget
    pub MyAppScreen = {{MyAppScreen}} {
        width: Fill, height: Fill
        flow: Down
        padding: 20

        show_bg: true
        draw_bg: { color: (DARK_BG) }

        <Label> {
            text: "My App"
            draw_text: {
                text_style: <FONT_MEDIUM> { font_size: 24.0 }
                color: (TEXT_PRIMARY)
            }
        }

        content = <View> {
            width: Fill, height: Fill
            // Your app content here
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MyAppScreen {
    #[deref]
    view: View,
}

impl Widget for MyAppScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
```

---

## Shell Integration

### 5. Add to Workspace

Edit `examples/mofa-studio/Cargo.toml`:

```toml
[workspace]
members = [
    "mofa-widgets",
    "mofa-studio-shell",
    "apps/mofa-fm",
    "apps/mofa-settings",
    "apps/mofa-myapp",  # Add your app
]
```

### 6. Add Shell Dependency

Edit `mofa-studio-shell/Cargo.toml`:

```toml
[dependencies]
mofa-myapp = { path = "../apps/mofa-myapp" }
```

### 7. Register in Shell

Edit `mofa-studio-shell/src/app.rs`:

```rust
// Add imports
use mofa_myapp::MoFaMyApp;

// In live_design! macro - add widget type import
live_design! {
    use mofa_myapp::screen::MyAppScreen;  // Compile-time requirement
    // ...
}

// In LiveHook::after_new_from_doc - register app info
impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.app_registry.register(MoFaFMApp::info());
        self.app_registry.register(MoFaSettingsApp::info());
        self.app_registry.register(MoFaMyApp::info());  // Add this
    }
}

// In LiveRegister::live_register - register widgets
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // ...
        <MoFaMyApp as MofaApp>::live_design(cx);  // Add this
    }
}
```

---

## Optional Features

### Timer Management

If your app uses interval timers (animations, polling), add timer control methods to your screen's Ref type:

```rust
// src/screen.rs
use makepad_widgets::*;

#[derive(Live, LiveHook, Widget)]
pub struct MyAppScreen {
    #[deref]
    view: View,
    #[rust]
    update_timer: Timer,
}

impl MyAppScreen {
    fn start_animation(&mut self, cx: &mut Cx) {
        self.update_timer = cx.start_interval(0.05);  // 50ms interval
    }
}

// Add timer control methods to the auto-generated Ref type
impl MyAppScreenRef {
    /// Stop timers - call this when hiding the widget
    pub fn stop_timers(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            cx.stop_timer(inner.update_timer);
        }
    }

    /// Start timers - call this when showing the widget
    pub fn start_timers(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_timer = cx.start_interval(0.05);
        }
    }
}
```

**Shell Integration:**

The shell must call these methods when switching apps:

```rust
// In shell's app switch logic
fn show_my_app(&mut self, cx: &mut Cx) {
    self.ui.my_app_screen(ids!(...my_app)).start_timers(cx);
    self.ui.view(ids!(...my_app)).set_visible(cx, true);
}

fn hide_my_app(&mut self, cx: &mut Cx) {
    self.ui.my_app_screen(ids!(...my_app)).stop_timers(cx);
    self.ui.view(ids!(...my_app)).set_visible(cx, false);
}
```

**Reference**: See `mofa-fm/src/screen.rs` for a complete example with audio meter timers.

### Using Shared Widgets

Import widgets from `mofa-widgets`:

```rust
live_design! {
    use mofa_widgets::waveform_view::WaveformView;
    use mofa_widgets::led_gauge::LedGauge;
    use mofa_widgets::participant_panel::ParticipantPanel;

    pub MyAppScreen = {{MyAppScreen}} {
        // Use shared widgets
        waveform = <WaveformView> { }
        gauge = <LedGauge> { }
    }
}
```

---

## Project Structure

```
apps/mofa-myapp/
├── Cargo.toml
└── src/
    ├── lib.rs          # MofaApp impl, exports
    ├── screen.rs       # Main screen widget
    └── components.rs   # Optional: sub-components
```

### Recommended lib.rs Pattern

```rust
//! MoFA MyApp - Description of your app

pub mod screen;
// pub mod components;  // Optional: additional modules

// Re-export main widget for shell's live_design! macro
pub use screen::MyAppScreen;

use makepad_widgets::Cx;
use mofa_widgets::{MofaApp, AppInfo};

pub struct MoFaMyApp;

impl MofaApp for MoFaMyApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "My App",
            id: "mofa-myapp",
            description: "Description here",
        }
    }

    fn live_design(cx: &mut Cx) {
        screen::live_design(cx);
        // components::live_design(cx);  // If you have sub-components
    }
}

/// Backwards-compatible registration function
pub fn live_design(cx: &mut Cx) {
    MoFaMyApp::live_design(cx);
}
```

---

## Theme Integration

Always use the shared theme from `mofa_widgets::theme`:

```rust
live_design! {
    // Fonts
    use mofa_widgets::theme::FONT_REGULAR;
    use mofa_widgets::theme::FONT_MEDIUM;
    use mofa_widgets::theme::FONT_SEMIBOLD;
    use mofa_widgets::theme::FONT_BOLD;

    // Colors (Light mode)
    use mofa_widgets::theme::DARK_BG;
    use mofa_widgets::theme::PANEL_BG;
    use mofa_widgets::theme::ACCENT_BLUE;
    use mofa_widgets::theme::TEXT_PRIMARY;
    use mofa_widgets::theme::TEXT_SECONDARY;

    // Colors (Dark mode variants)
    use mofa_widgets::theme::DARK_BG_DARK;
    use mofa_widgets::theme::PANEL_BG_DARK;
    use mofa_widgets::theme::TEXT_PRIMARY_DARK;
    use mofa_widgets::theme::TEXT_SECONDARY_DARK;
}
```

**Do NOT** define fonts or colors locally in your app.

---

## Dark Mode Support

MoFA Studio supports runtime dark/light mode switching. Apps should implement dark mode to maintain visual consistency.

### Adding Dark Mode to Widgets

Use `instance dark_mode` with `mix()` in shaders:

```rust
live_design! {
    use mofa_widgets::theme::*;

    pub MyWidget = {{MyWidget}} <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0  // 0.0=light, 1.0=dark

            fn get_color(self) -> vec4 {
                return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
            }
        }

        label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }
}
```

### Adding update_dark_mode Method

Implement on your screen's Ref type so the shell can propagate theme changes:

```rust
impl MyAppScreenRef {
    /// Update dark mode for this screen
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Update panel backgrounds
            inner.view.apply_over(cx, live!{
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Update labels
            inner.view.label(ids!(header.title)).apply_over(cx, live!{
                draw_text: { dark_mode: (dark_mode) }
            });

            inner.view.redraw(cx);
        }
    }
}
```

### Shell Integration (CRITICAL)

**You MUST register your app's dark mode update in the shell.** Without this, your app won't respond to theme changes.

Edit `mofa-studio-shell/src/app.rs`:

1. **Import the WidgetRefExt trait:**
```rust
use mofa_myapp::{MoFaMyApp, MyAppScreenWidgetRefExt};
```

2. **Add to `apply_dark_mode_screens_with_value()`:**
```rust
fn apply_dark_mode_screens_with_value(&mut self, cx: &mut Cx, dm: f64) {
    // Existing screens...
    self.ui.mo_fa_fmscreen(ids!(...fm_page)).update_dark_mode(cx, dm);

    // ADD YOUR APP HERE - this is required!
    self.ui.my_app_screen(ids!(...my_app_page)).update_dark_mode(cx, dm);
}
```

**Common mistake:** Forgetting to add this line means your app's dark mode shaders are defined but never receive the `dark_mode` value updates.

### Important: vec4 in apply_over

**Hex colors do NOT work in `apply_over` at runtime!** Use `vec4()` format:

```rust
// ❌ FAILS - hex colors don't work in apply_over
self.view.apply_over(cx, live!{ draw_bg: { color: #1f293b } });

// ✅ WORKS - vec4 format
self.view.apply_over(cx, live!{ draw_bg: { color: (vec4(0.12, 0.16, 0.23, 1.0)) } });
```

### Color Reference (vec4 format)

| Purpose | Light Mode | Dark Mode |
|---------|------------|-----------|
| Panel background | `vec4(1.0, 1.0, 1.0, 1.0)` | `vec4(0.12, 0.16, 0.23, 1.0)` |
| Text primary | `vec4(0.12, 0.16, 0.22, 1.0)` | `vec4(0.95, 0.96, 0.98, 1.0)` |
| Hover background | `vec4(0.95, 0.96, 0.98, 1.0)` | `vec4(0.2, 0.25, 0.33, 1.0)` |

---

## Checklist

Before submitting your app:

**MofaApp Trait:**
- [ ] Implements `MofaApp` trait with valid `info()` and `live_design()`
- [ ] Exports main screen widget for shell's `live_design!` macro

**mofa-ui (Mandatory):**
- [ ] Uses `mofa-ui` dependency for shared widgets (MofaHero, AudioManager, LedMeter, etc.)
- [ ] MofaHeroAction uses `find_widget_action_cast` scoped by `widget_uid()` — NEVER global cast
- [ ] Calls `set_running(cx, true/false)` on start, stop, DataflowStopped, and Error
- [ ] Audio capture is serialized: stop CPAL mic monitoring before AEC start, restart on stop

**moly-kit Chat UI (if applicable):**
- [ ] Registers `moly_kit::widgets::live_design(cx)` BEFORE screen registration
- [ ] Uses `BotId::new("name", "provider")` — NEVER `BotId::default()`
- [ ] ChatController initialized in `draw_walk` (before first render)
- [ ] Mutex guard dropped before any UI calls (`redraw`, `instant_scroll_to_bottom`)
- [ ] Uses `ids!()` not `id!()` for Messages widget access

**Theme & Dark Mode:**
- [ ] Uses shared theme (no local font/color definitions)
- [ ] Widgets have `instance dark_mode: 0.0` for themeable elements
- [ ] Implements `update_dark_mode()` on screen Ref type
- [ ] Uses `vec4()` for runtime color changes in `apply_over()`

**Timer Management (if applicable):**
- [ ] Implements `stop_timers()` and `start_timers()` on Ref type
- [ ] Shell calls timer methods when hiding/showing app

**Integration:**
- [ ] Added to workspace `Cargo.toml`
- [ ] Added as dependency in `mofa-studio-shell/Cargo.toml`
- [ ] Registered in shell's `LiveHook::after_new_from_doc`
- [ ] Registered in shell's `LiveRegister::live_register`
- [ ] Widget type imported in shell's `live_design!` macro
- [ ] **WidgetRefExt trait imported** in shell (e.g., `use mofa_myapp::MyAppScreenWidgetRefExt`)
- [ ] **Added to `apply_dark_mode_screens_with_value()`** in shell's app.rs
- [ ] `cargo build` passes with no errors

---

## Mandatory: mofa-ui Shared Components

All apps MUST use `mofa-ui` for shared UI infrastructure. Do NOT reimplement these locally.

### Required Dependency

```toml
# Cargo.toml
[dependencies]
mofa-ui = { path = "../../mofa-ui" }
```

### Required Imports

```rust
// MofaHero (start/stop button with connection status)
use mofa_ui::{MofaHeroWidgetExt, MofaHeroAction, ConnectionStatus};

// Audio infrastructure
use mofa_ui::{AudioManager, AudioDeviceInfo};
use mofa_ui::{LedMeterWidgetExt, MicButtonWidgetExt, AecButtonWidgetExt};
```

### MofaHero Action Handling (CRITICAL)

**NEVER** use global action cast. Every app in the shell shares the same event loop. Using `action.as_widget_action().cast()` matches MofaHeroAction from ALL MofaHero widgets globally — clicking Start on one app triggers all apps.

```rust
// ❌ BAD — matches ANY MofaHero in the entire shell
for action in actions {
    match action.as_widget_action().cast() {
        MofaHeroAction::StartClicked => { /* ALL apps fire */ }
    }
}

// ✅ GOOD — scoped to this screen's own hero widget
let hero_uid = self.view.mofa_hero(ids!(left_column.mofa_hero)).widget_uid();
match actions.find_widget_action_cast::<MofaHeroAction>(hero_uid) {
    MofaHeroAction::StartClicked => { /* only THIS app fires */ }
    MofaHeroAction::StopClicked => { /* ... */ }
    MofaHeroAction::None => {}
}
```

### MofaHero State Management

You MUST call `set_running()` to toggle the button visual state:

```rust
fn handle_start(&mut self, cx: &mut Cx) {
    self.view.mofa_hero(ids!(left_column.mofa_hero)).set_running(cx, true);
    self.view.mofa_hero(ids!(left_column.mofa_hero)).set_connection_status(cx, ConnectionStatus::Connecting);
    // ... start dataflow
}

fn handle_stop(&mut self, cx: &mut Cx) {
    self.view.mofa_hero(ids!(left_column.mofa_hero)).set_running(cx, false);
    self.view.mofa_hero(ids!(left_column.mofa_hero)).set_connection_status(cx, ConnectionStatus::Stopping);
    // ... stop dataflow
}
```

Also update on `DataflowStopped` and `Error` events — always set `set_running(cx, false)`.

### Audio: Dual Capture Prevention

macOS does not reliably support two concurrent audio input streams. If your app uses both CPAL mic monitoring (for UI level meters) and AEC capture (for Dora pipeline), you MUST serialize them:

```rust
DoraEvent::DataflowStarted { .. } => {
    // Stop CPAL mic monitoring BEFORE starting AEC capture
    if let Some(ref mut manager) = self.audio_manager {
        manager.stop_mic_monitoring();
    }
    if let Some(ref dora) = self.dora_integration {
        dora.set_aec_enabled(true);
        dora.start_recording();
    }
}

DoraEvent::DataflowStopped => {
    // Restart CPAL mic monitoring after AEC stops
    if let Some(ref mut manager) = self.audio_manager {
        let _ = manager.start_mic_monitoring(None);
    }
}
```

---

## Using moly-kit Chat UI

For chat/message display, use moly-kit's `Messages` widget with `ChatController`.

### Setup

```toml
# Cargo.toml
[dependencies]
moly-kit = { git = "https://github.com/moxin-org/moly", branch = "main" }
```

```rust
// lib.rs — register moly-kit widgets BEFORE your screen
fn live_design(cx: &mut Cx) {
    moly_kit::widgets::live_design(cx);
    screen::live_design(cx);
}
```

```rust
// design.rs — use Messages widget
live_design! {
    use moly_kit::widgets::messages::Messages;

    pub MyScreen = {{MyScreen}} {
        chat_messages = <Messages> { width: Fill, height: Fill }
    }
}
```

### ChatController Pattern

```rust
use std::sync::{Arc, Mutex};
use moly_kit::prelude::*;

#[derive(Live, LiveHook, Widget)]
pub struct MyScreen {
    #[deref] view: View,
    #[rust] chat_controller: Option<Arc<Mutex<ChatController>>>,
    #[rust] last_chat_count: usize,
}
```

### BotId: NEVER use Default (CRITICAL)

`BotId::default()` creates an empty string that **panics** inside moly-kit's `draw_list` when it tries to parse the bot id format (`<len>;<id>@<provider>`). This panic occurs in a macOS `extern "C"` timer callback and causes an unrecoverable abort.

```rust
// ❌ CRASH — BotId::default() is empty string "", panics in draw_list
EntityId::Bot(BotId::default())

// ✅ CORRECT — properly formatted bot id
EntityId::Bot(BotId::new("asr", "local"))
EntityId::Bot(BotId::new("tutor", "local"))
```

### Initialize ChatController in draw_walk

The Messages widget panics if `chat_controller` is `None` during draw. Initialize it in `draw_walk` to guarantee it's set before the first render:

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    if self.chat_controller.is_none() {
        let controller = ChatController::new_arc();
        self.chat_controller = Some(controller.clone());
        self.view.messages(ids!(chat_messages)).write().chat_controller = Some(controller);
    }
    self.view.draw_walk(cx, scope, walk)
}
```

### Syncing Messages from Dora State

```rust
fn sync_chat_from_dora(&mut self, cx: &mut Cx, messages: Vec<ChatMessage>) {
    let controller = self.chat_controller.clone().unwrap();

    // Lock, update, drop — BEFORE any UI calls
    let count = {
        let mut guard = controller.lock().expect("ChatController mutex poisoned");
        let state = guard.dangerous_state_mut();
        state.messages.clear();
        for msg in &messages {
            let entity = match msg.role {
                MessageRole::User => EntityId::User,
                _ => EntityId::Bot(BotId::new("asr", "local")), // NEVER BotId::default()
            };
            state.messages.push(Message {
                from: entity,
                content: MessageContent { text: msg.content.clone(), ..Default::default() },
                ..Default::default()
            });
        }
        state.messages.len()
    }; // guard dropped here

    if count > self.last_chat_count {
        self.last_chat_count = count;
        self.view.messages(ids!(chat_messages)).write().instant_scroll_to_bottom(cx);
    }
    self.view.redraw(cx);
}
```

### Mutex Safety Rules

The `ChatController` mutex is shared between timer callbacks (write) and draw passes (read). On macOS, timer callbacks are `extern "C"` and **cannot unwind** — any panic is a fatal abort.

1. **Always drop the lock before calling UI methods** (`redraw`, `instant_scroll_to_bottom`) — these can trigger draws that re-lock
2. **Use `ids!()` not `id!()`** for Messages widget access — `messages()` takes `&[LiveId]`
3. **Use `instant_scroll_to_bottom`** not `animated_scroll_to_bottom` — animated triggers intermediate redraws

---

## Reference Apps

| App | Description | Features |
|-----|-------------|----------|
| `mofa-fm` | Audio streaming | Timer management, shader animations, Dora integration |
| `mofa-asr` | Speech recognition | moly-kit chat UI, AEC capture, dual capture prevention |
| `mofa-debate` | Multi-agent debate | Audio playback, multiple participants |
| `mofa-settings` | Provider config | Modal dialogs, form inputs, state management |

---

## Troubleshooting

### "no function named `live_design_with`"

Your widget type isn't properly imported in the shell's `live_design!` macro:
```rust
live_design! {
    use mofa_myapp::screen::MyAppScreen;  // Must be here
}
```

### "trait bound `MoFaMyApp: MofaApp` is not satisfied"

Check your imports:
```rust
use mofa_widgets::{MofaApp, AppInfo};  // Both needed
```

### Timer keeps running when app is hidden

Implement timer control and ensure shell calls `stop_timers()` on visibility change.

### Clicking Start triggers multiple apps

You're using global action cast. Use `find_widget_action_cast` scoped to your widget's `widget_uid()`. See "MofaHero Action Handling" above.

### Fatal crash (abort) in macOS timer callback

Any panic inside a timer callback is fatal on macOS (`extern "C"` cannot unwind). Common causes:
- `BotId::default()` — use `BotId::new("id", "provider")` instead
- `expect("no chat controller set")` — initialize ChatController in `draw_walk`
- Mutex locked during UI call that triggers redraw — drop guard before UI calls

To debug: add a panic hook in `main()` to capture the real panic message:
```rust
std::panic::set_hook(Box::new(|info| {
    eprintln!("=== PANIC: {} ===", info);
    eprintln!("{}", std::backtrace::Backtrace::force_capture());
}));
```

### AEC not receiving audio / mic not working

Ensure you call `set_aec_enabled(true)` and `start_recording()` on `DataflowStarted`. Also check for dual capture — stop CPAL mic monitoring before AEC starts.

### Start/Stop button doesn't toggle visually

Missing `set_running(cx, true/false)` calls. Must be called in handle_start, handle_stop, DataflowStopped, and Error handlers.

### Fonts/colors don't match other apps

Use `mofa_widgets::theme::*` instead of defining locally.

### Dark mode doesn't work on my app

Check these in order:
1. **Shaders have `instance dark_mode: 0.0`** - Each drawable element needs this
2. **Screen has `update_dark_mode()` method** - Must be implemented on the Ref type
3. **WidgetRefExt trait is imported in shell** - `use mofa_myapp::MyAppScreenWidgetRefExt;`
4. **Screen is registered in `apply_dark_mode_screens_with_value()`** - This is the most common miss!

---

*Last Updated: 2026-01-27*
