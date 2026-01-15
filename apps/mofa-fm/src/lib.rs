//! MoFA FM App - AI-powered audio streaming and voice interface

pub mod audio;
pub mod audio_player;
pub mod dora_integration;
pub mod log_bridge;
pub mod mofa_hero;
pub mod screen;
pub mod system_monitor;

pub use audio::AudioManager;
pub use dora_integration::{DoraCommand, DoraEvent, DoraIntegration};
pub use mofa_hero::{ConnectionStatus, MofaHero, MofaHeroAction};
pub use screen::MoFaFMScreen;
pub use screen::MoFaFMScreenWidgetRefExt; // Export WidgetRefExt for timer control

use makepad_widgets::Cx;
use mofa_widgets::{AppInfo, MofaApp};

/// MoFA FM app descriptor
pub struct MoFaFMApp;

impl MofaApp for MoFaFMApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "MoFA FM",
            id: "mofa-fm",
            description: "AI-powered audio streaming and voice interface",
        }
    }

    fn live_design(cx: &mut Cx) {
        mofa_hero::live_design(cx);
        screen::live_design(cx);
    }
}

/// Register all MoFA FM widgets with Makepad
/// (Kept for backwards compatibility - calls DoraApp::live_design)
pub fn live_design(cx: &mut Cx) {
    MoFaFMApp::live_design(cx);
}
