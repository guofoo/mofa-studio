//! StepAudio2 ASR engine wrapper
//!
//! Wraps the step-audio2-mlx library for multi-language ASR on Apple Silicon.
//! Uses Whisper-style encoder + Qwen2.5-7B LLM for high-quality transcription.

use eyre::{Context, Result};
use std::path::Path;

use step_audio2_mlx::StepAudio2;

/// StepAudio2 ASR engine for multi-language speech recognition
pub struct StepAudio2Engine {
    model: StepAudio2,
}

impl StepAudio2Engine {
    /// Create a new StepAudio2 engine
    ///
    /// # Arguments
    /// * `model_dir` - Path to model directory containing config.json, tokenizer.json, model weights
    pub fn new(model_dir: impl AsRef<Path>) -> Result<Self> {
        let model_dir = model_dir.as_ref();

        log::info!("Loading StepAudio2 model from {:?}", model_dir);

        let mut model =
            StepAudio2::load(model_dir).context("Failed to load StepAudio2 model")?;

        log::info!("Warming up StepAudio2 model...");
        model.warmup().context("Failed to warm up StepAudio2 model")?;

        log::info!("StepAudio2 model loaded successfully");

        Ok(Self { model })
    }

    /// Transcribe audio samples to text
    ///
    /// # Arguments
    /// * `samples` - Audio samples as f32 slice (any sample rate, resampled internally to 16kHz)
    /// * `sample_rate` - Sample rate of input audio
    ///
    /// # Returns
    /// Tuple of (transcribed text, detected language)
    pub fn transcribe(&mut self, samples: &[f32], sample_rate: u32) -> Result<(String, String)> {
        let text = self
            .model
            .transcribe_samples(samples, sample_rate)
            .context("Transcription failed")?;

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
            chinese_count += 1;
        } else if (c >= '\u{3040}' && c <= '\u{309F}') || (c >= '\u{30A0}' && c <= '\u{30FF}') {
            japanese_count += 1;
        } else if c.is_ascii_alphabetic() {
            english_count += 1;
        }
    }

    if japanese_count > 0 {
        return "ja".to_string();
    }
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
