//! SenseVoice ASR engine using funasr-nano-mlx
//!
//! This engine wraps the funasr-nano-mlx library for multi-language ASR on Apple Silicon.
//! It uses the SenseVoice encoder + Qwen3-0.6B LLM for high-quality transcription.

use eyre::{Context, Result};
use std::path::Path;

use funasr_nano_mlx::model::FunASRNano;

/// SenseVoice ASR engine for multi-language speech recognition
pub struct SenseVoiceEngine {
    model: FunASRNano,
}

impl SenseVoiceEngine {
    /// Create a new SenseVoice engine
    ///
    /// # Arguments
    /// * `model_dir` - Path to model directory containing:
    ///   - `config.yaml` - Model configuration
    ///   - `model.safetensors` - Model weights
    ///   - `Qwen3-0.6B/` - Qwen3 tokenizer and config
    pub fn new(model_dir: impl AsRef<Path>) -> Result<Self> {
        let model_dir = model_dir.as_ref();

        log::info!("Loading SenseVoice model from {:?}", model_dir);

        let model = FunASRNano::load(model_dir).context("Failed to load SenseVoice model")?;

        log::info!("SenseVoice model loaded successfully");

        Ok(Self { model })
    }

    /// Transcribe audio samples to text
    ///
    /// # Arguments
    /// * `samples` - Audio samples as f32 slice (any sample rate, will be resampled to 16kHz)
    /// * `sample_rate` - Sample rate of input audio
    ///
    /// # Returns
    /// Tuple of (transcribed text, detected language)
    pub fn transcribe(&mut self, samples: &[f32], sample_rate: u32) -> Result<(String, String)> {
        // The funasr-nano-mlx library handles resampling internally
        let text = self
            .model
            .transcribe_samples(samples, sample_rate)
            .context("Transcription failed")?;

        // Detect language from output (simple heuristic based on character ranges)
        let language = detect_language(&text);

        Ok((text, language))
    }
}

/// Simple language detection based on character ranges
fn detect_language(text: &str) -> String {
    let mut chinese_count = 0;
    let mut japanese_count = 0;
    let mut english_count = 0;

    for c in text.chars() {
        if c >= '\u{4E00}' && c <= '\u{9FFF}' {
            // CJK Unified Ideographs (shared by Chinese and Japanese)
            chinese_count += 1;
        } else if (c >= '\u{3040}' && c <= '\u{309F}') || (c >= '\u{30A0}' && c <= '\u{30FF}') {
            // Hiragana or Katakana (Japanese specific)
            japanese_count += 1;
        } else if c.is_ascii_alphabetic() {
            english_count += 1;
        }
    }

    // If Japanese-specific characters found, it's Japanese
    if japanese_count > 0 {
        return "ja".to_string();
    }

    // Otherwise, compare Chinese vs English
    if chinese_count > english_count {
        "zh".to_string()
    } else if english_count > 0 {
        "en".to_string()
    } else if chinese_count > 0 {
        "zh".to_string()
    } else {
        "unknown".to_string()
    }
}
