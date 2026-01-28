//! Paraformer ASR engine using funasr-mlx
//!
//! This engine wraps the funasr-mlx library for Chinese ASR on Apple Silicon.
//! It uses the Paraformer-large model which provides fast, accurate transcription.

use eyre::{Context, Result};
use std::path::Path;

use funasr_mlx::{load_model, parse_cmvn_file, transcribe, Paraformer, Vocabulary};
use funasr_mlx::punctuation::PunctuationModel;

/// Paraformer ASR engine for Chinese speech recognition
pub struct ParaformerEngine {
    model: Paraformer,
    vocab: Vocabulary,
    punc_model: Option<PunctuationModel>,
}

impl ParaformerEngine {
    /// Create a new Paraformer engine
    ///
    /// # Arguments
    /// * `model_dir` - Path to model directory containing:
    ///   - `paraformer.safetensors` - Model weights
    ///   - `am.mvn` - CMVN normalization parameters
    ///   - `tokens.txt` - Vocabulary file
    pub fn new(model_dir: impl AsRef<Path>) -> Result<Self> {
        let model_dir = model_dir.as_ref();

        log::info!("Loading Paraformer model from {:?}", model_dir);

        // Load model weights
        let weights_path = model_dir.join("paraformer.safetensors");
        let mut model =
            load_model(&weights_path).context("Failed to load Paraformer model weights")?;

        // Load and apply CMVN normalization
        let cmvn_path = model_dir.join("am.mvn");
        let (addshift, rescale) =
            parse_cmvn_file(&cmvn_path).context("Failed to parse CMVN file")?;
        model.set_cmvn(addshift, rescale);

        // Load vocabulary
        let vocab_path = model_dir.join("tokens.txt");
        let vocab = Vocabulary::load(&vocab_path).context("Failed to load vocabulary")?;

        log::info!(
            "Paraformer model loaded successfully (vocab size: {})",
            vocab.len()
        );

        // Load punctuation model if enabled
        let enable_punc = std::env::var("ENABLE_PUNCTUATION")
            .map(|v| v.to_lowercase() != "false")
            .unwrap_or(true);

        let punc_model = if enable_punc {
            let punc_dir = std::env::var("FUNASR_PUNC_MODEL_DIR")
                .map(|p| std::path::PathBuf::from(p))
                .unwrap_or_else(|_| {
                    dirs_next::home_dir()
                        .unwrap_or_default()
                        .join(".dora/models/asr/funasr/punc_ct-transformer_cn-en-common-vocab471067-large")
                });

            if punc_dir.exists() {
                match PunctuationModel::load(&punc_dir) {
                    Ok(m) => {
                        log::info!("Punctuation model loaded from {:?}", punc_dir);
                        Some(m)
                    }
                    Err(e) => {
                        log::warn!("Failed to load punctuation model: {}. Continuing without punctuation.", e);
                        None
                    }
                }
            } else {
                log::info!("Punctuation model directory not found at {:?}. Continuing without punctuation.", punc_dir);
                None
            }
        } else {
            log::info!("Punctuation restoration disabled via ENABLE_PUNCTUATION=false");
            None
        };

        Ok(Self { model, vocab, punc_model })
    }

    /// Transcribe audio samples to text
    ///
    /// # Arguments
    /// * `samples` - Audio samples as f32 slice (16kHz, mono, normalized to [-1, 1])
    ///
    /// # Returns
    /// Transcribed text in Chinese
    pub fn transcribe(&mut self, samples: &[f32]) -> Result<String> {
        let text = if let Some(ref mut punc) = self.punc_model {
            funasr_mlx::transcribe_with_punctuation(&mut self.model, samples, &self.vocab, punc)
                .context("Transcription with punctuation failed")?
        } else {
            transcribe(&mut self.model, samples, &self.vocab).context("Transcription failed")?
        };

        Ok(text)
    }

    /// Get supported languages
    pub fn supported_languages(&self) -> &[&str] {
        &["zh"]
    }
}
