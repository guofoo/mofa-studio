//! Configuration for dora-asr-nano-mlx node

use std::env;
use std::path::PathBuf;

/// Configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to SenseVoice model directory
    pub model_dir: PathBuf,
    /// Target language: "zh", "en", "ja", "auto" (default: auto)
    pub language: String,
    /// Minimum audio duration in seconds (default: 0.1)
    pub min_audio_duration: f64,
    /// Maximum audio duration in seconds (default: 30.0)
    pub max_audio_duration: f64,
    /// Pre-initialize model on startup (default: true)
    pub warmup: bool,
    /// Log level (default: INFO)
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let default_model_dir = format!("{}/.mofa/models/Fun-ASR-Nano-2512", home);

        Self {
            model_dir: PathBuf::from(
                env::var("SENSEVOICE_MODEL_DIR").unwrap_or(default_model_dir),
            ),
            language: env::var("ASR_NANO_LANGUAGE").unwrap_or_else(|_| "auto".to_string()),
            min_audio_duration: env::var("MIN_AUDIO_DURATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.1),
            max_audio_duration: env::var("MAX_AUDIO_DURATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30.0),
            warmup: env::var("ASR_MLX_WARMUP")
                .map(|s| s.to_lowercase() != "false" && s != "0")
                .unwrap_or(true),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()),
        }
    }
}
