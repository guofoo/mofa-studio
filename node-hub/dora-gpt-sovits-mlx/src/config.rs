//! Configuration for dora-gpt-sovits-mlx node
//!
//! All settings can be overridden via environment variables.
//!
//! Supports voice character presets via VOICE_CHARACTER env var,
//! which loads settings from ~/.dora/models/primespeech/voices.json

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Default voices config location
const DEFAULT_VOICES_CONFIG: &str = "~/.dora/models/primespeech/voices.json";

/// Voice configuration from JSON
#[derive(Debug, Deserialize, Clone)]
pub struct VoiceConfig {
    pub ref_audio: String,
    pub ref_text: String,
    #[serde(default)]
    pub vits_onnx: Option<String>,
    #[serde(default)]
    pub codes_path: Option<String>,
    #[serde(default)]
    pub speed_factor: Option<f32>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Root voices.json configuration structure
#[derive(Debug, Deserialize)]
pub struct VoicesConfig {
    #[serde(default = "default_voice")]
    pub default_voice: String,
    #[serde(default = "default_base_path")]
    pub models_base_path: String,
    pub voices: BTreeMap<String, VoiceConfig>,
}

fn default_voice() -> String {
    "doubao".to_string()
}

fn default_base_path() -> String {
    "~/.dora/models/primespeech".to_string()
}

impl VoicesConfig {
    /// Load voices config from JSON file
    pub fn load(path: &str) -> Option<Self> {
        let expanded = expand_path(path);
        let content = std::fs::read_to_string(&expanded).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Get config file path from env var or default
    pub fn config_path() -> String {
        std::env::var("VOICES_CONFIG").unwrap_or_else(|_| DEFAULT_VOICES_CONFIG.to_string())
    }

    /// Find a voice by name or alias
    pub fn find_voice(&self, name: &str) -> Option<&VoiceConfig> {
        let name_lower = name.to_lowercase();

        // Direct match
        if let Some(voice) = self.voices.get(&name_lower) {
            return Some(voice);
        }

        // Search aliases
        for voice in self.voices.values() {
            if voice.aliases.iter().any(|a| a.to_lowercase() == name_lower) {
                return Some(voice);
            }
        }

        None
    }

    /// Resolve a relative path to absolute using base_path
    pub fn resolve_path(&self, relative: &str) -> String {
        if relative.starts_with('/') || relative.starts_with('~') {
            expand_path(relative)
        } else {
            let base = expand_path(&self.models_base_path);
            format!("{}/{}", base, relative)
        }
    }
}

/// Expand ~ to home directory
fn expand_path(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs_next::home_dir() {
            return format!("{}{}", home.display(), &path[1..]);
        }
    }
    path.to_string()
}

/// Node configuration
#[derive(Debug, Clone)]
pub struct Config {
    // Model paths
    pub model_dir: String,
    pub t2s_weights: String,
    pub bert_weights: String,
    pub bert_tokenizer: String,
    pub vits_weights: String,
    /// Path to pretrained VITS weights (base model for finetuned weights)
    /// When set, vits_weights is treated as finetuned overlay on this pretrained base
    pub vits_pretrained_base: Option<String>,
    pub hubert_weights: String,
    pub vits_onnx_path: Option<String>,

    // Reference audio
    pub ref_audio: String,
    pub ref_text: Option<String>,
    /// Path to pre-computed semantic codes (for few-shot mode)
    pub codes_path: Option<String>,

    // Voice character name (if loaded from voices.json)
    pub voice_character: Option<String>,

    // Synthesis parameters
    pub sample_rate: u32,
    pub speed: f32,
    pub top_k: i32,
    pub top_p: f32,
    pub temperature: f32,
    pub repetition_penalty: f32,
    pub noise_scale: f32,
    pub use_mlx_vits: bool,

    // Synthesis options
    pub synthesis_timeout_secs: Option<u64>,

    // Streaming options
    /// Enable streaming mode (process each sentence separately)
    pub return_fragment: bool,
    /// Minimum characters before emitting a fragment (default: 10)
    pub fragment_min_chars: usize,
    /// Silence duration (seconds) between audio fragments (default: 0.3)
    pub fragment_interval: f32,
}

impl Config {
    pub fn from_env() -> Self {
        let home = dirs_next::home_dir().unwrap_or_default();

        // Check for VOICE_CHARACTER env var first
        let voice_character = std::env::var("VOICE_CHARACTER").ok();
        let voices_config = VoicesConfig::load(&VoicesConfig::config_path());

        // Load voice preset if VOICE_CHARACTER is set
        let voice_preset: Option<(VoiceConfig, String)> = voice_character.as_ref().and_then(|name| {
            voices_config.as_ref().and_then(|config| {
                config.find_voice(name).map(|v| {
                    let resolved_ref_audio = config.resolve_path(&v.ref_audio);
                    let resolved_vits_onnx = v.vits_onnx.as_ref().map(|p| config.resolve_path(p));
                    let resolved_codes = v.codes_path.as_ref().map(|p| config.resolve_path(p));
                    (
                        VoiceConfig {
                            ref_audio: resolved_ref_audio,
                            ref_text: v.ref_text.clone(),
                            vits_onnx: resolved_vits_onnx,
                            codes_path: resolved_codes,
                            speed_factor: v.speed_factor,
                            aliases: v.aliases.clone(),
                        },
                        config.models_base_path.clone(),
                    )
                })
            })
        });

        if let Some((ref preset, _)) = voice_preset {
            log::info!(
                "Loaded voice character '{}': ref_audio={}, vits_onnx={:?}",
                voice_character.as_deref().unwrap_or("unknown"),
                preset.ref_audio,
                preset.vits_onnx
            );
        } else if voice_character.is_some() {
            log::warn!(
                "Voice character '{}' not found in voices.json",
                voice_character.as_deref().unwrap_or("")
            );
        }

        // Model directory (default: ~/.dora/models/primespeech/gpt-sovits-mlx)
        let model_dir = std::env::var("GPT_SOVITS_MODEL_DIR").unwrap_or_else(|_| {
            home.join(".dora/models/primespeech/gpt-sovits-mlx")
                .to_string_lossy()
                .to_string()
        });
        let model_path = PathBuf::from(&model_dir);

        // Individual model paths (can override defaults)
        let t2s_weights = std::env::var("T2S_WEIGHTS").unwrap_or_else(|_| {
            model_path
                .join("doubao_mixed_gpt_new.safetensors")
                .to_string_lossy()
                .to_string()
        });

        let bert_weights = std::env::var("BERT_WEIGHTS").unwrap_or_else(|_| {
            model_path
                .join("bert.safetensors")
                .to_string_lossy()
                .to_string()
        });

        let bert_tokenizer = std::env::var("BERT_TOKENIZER").unwrap_or_else(|_| {
            model_path
                .join("chinese-roberta-tokenizer/tokenizer.json")
                .to_string_lossy()
                .to_string()
        });

        let vits_weights = std::env::var("VITS_WEIGHTS").unwrap_or_else(|_| {
            model_path
                .join("doubao_mixed_sovits_new.safetensors")
                .to_string_lossy()
                .to_string()
        });

        let hubert_weights = std::env::var("HUBERT_WEIGHTS").unwrap_or_else(|_| {
            model_path
                .join("hubert.safetensors")
                .to_string_lossy()
                .to_string()
        });

        // VITS ONNX path: env var > voice preset > default
        let vits_onnx_path = std::env::var("VITS_ONNX_PATH").ok().or_else(|| {
            voice_preset
                .as_ref()
                .and_then(|(v, _)| v.vits_onnx.clone())
        }).or_else(|| {
            let path = model_path.join("vits.onnx");
            if path.exists() {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        });

        // Pretrained VITS base (for finetuned weights overlay)
        let vits_pretrained_base = std::env::var("VITS_PRETRAINED_BASE").ok();

        // Reference audio: env var > voice preset > default
        let ref_audio = std::env::var("REF_AUDIO").unwrap_or_else(|_| {
            voice_preset
                .as_ref()
                .map(|(v, _)| v.ref_audio.clone())
                .unwrap_or_else(|| {
                    home.join(".dora/models/primespeech/moyoyo/ref_audios/doubao_ref_mix_new.wav")
                        .to_string_lossy()
                        .to_string()
                })
        });

        // Reference text: env var > voice preset > None
        let ref_text = std::env::var("REF_TEXT").ok().or_else(|| {
            voice_preset.as_ref().map(|(v, _)| v.ref_text.clone())
        });

        // Codes path: env var > voice preset > None
        let codes_path = std::env::var("CODES_PATH").ok().or_else(|| {
            voice_preset.as_ref().and_then(|(v, _)| v.codes_path.clone())
        });

        // Synthesis parameters
        let sample_rate = std::env::var("SAMPLE_RATE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(32000);

        // Speed: env var > voice preset > default (1.0)
        let speed = std::env::var("SPEED_FACTOR")
            .ok()
            .and_then(|s| s.parse().ok())
            .or_else(|| voice_preset.as_ref().and_then(|(v, _)| v.speed_factor))
            .unwrap_or(1.0);

        let top_k = std::env::var("TOP_K")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let top_p = std::env::var("TOP_P")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.0);

        let temperature = std::env::var("TEMPERATURE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.0);

        let repetition_penalty = std::env::var("REPETITION_PENALTY")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.35);

        let noise_scale = std::env::var("NOISE_SCALE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.5);

        let use_mlx_vits = std::env::var("USE_MLX_VITS")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let synthesis_timeout_secs = std::env::var("SYNTHESIS_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok());

        // Streaming options
        let return_fragment = std::env::var("RETURN_FRAGMENT")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let fragment_min_chars = std::env::var("FRAGMENT_MIN_CHARS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let fragment_interval = std::env::var("FRAGMENT_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.3);

        Self {
            model_dir,
            t2s_weights,
            bert_weights,
            bert_tokenizer,
            vits_weights,
            vits_pretrained_base,
            hubert_weights,
            vits_onnx_path,
            ref_audio,
            ref_text,
            codes_path,
            voice_character,
            sample_rate,
            speed,
            top_k,
            top_p,
            temperature,
            repetition_penalty,
            noise_scale,
            use_mlx_vits,
            synthesis_timeout_secs,
            return_fragment,
            fragment_min_chars,
            fragment_interval,
        }
    }
}
