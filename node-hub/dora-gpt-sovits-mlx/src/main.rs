//! Dora TTS Node using MLX-based GPT-SoVITS
//!
//! This node provides text-to-speech synthesis optimized for Apple Silicon
//! using the GPT-SoVITS model via MLX (Metal acceleration).
//!
//! Interface compatible with dora-primespeech:
//! - Input `text`: Arrow string array with metadata (question_id, session_status)
//! - Output `audio`: Arrow float32 array with metadata (sample_rate, duration, question_id)
//! - Output `segment_complete`: Arrow string ("completed"/"error") with metadata
//!
//! Streaming mode (RETURN_FRAGMENT=true):
//! - Splits text into sentences and processes each separately
//! - Emits audio after each sentence for lower latency
//! - Emits segment_complete only after all sentences are done

use dora_node_api::{
    arrow::array::{Array, Float32Array, StringArray},
    dora_core::config::NodeId,
    DoraNode, Event, Parameter,
};
use eyre::Result;
use gpt_sovits_mlx::voice_clone::{VoiceCloner, VoiceClonerConfig, SynthesisOptions};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// Split text into sentences for streaming synthesis.
///
/// Splits on Chinese/English sentence-ending punctuation while preserving
/// the punctuation in each segment.
fn split_into_sentences(text: &str, min_chars: usize) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();

    for c in text.chars() {
        current.push(c);

        // Check for sentence-ending punctuation
        let is_sentence_end = matches!(c,
            '。' | '！' | '？' | '；' |  // Chinese
            '.' | '!' | '?' | ';'       // English
        );

        if is_sentence_end && current.len() >= min_chars {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                sentences.push(trimmed);
            }
            current = String::new();
        }
    }

    // Don't forget the last segment (may not end with punctuation)
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        // If last segment is too short, merge with previous
        if trimmed.len() < min_chars && !sentences.is_empty() {
            let last = sentences.pop().unwrap();
            sentences.push(format!("{}{}", last, trimmed));
        } else {
            sentences.push(trimmed);
        }
    }

    sentences
}

mod config;
mod ssml;
use config::Config;
use ssml::{is_ssml, parse_ssml, SsmlSegment};

/// Normalize text for TTS: replace curly quotes, special punctuation, etc.
/// that the G2P pipeline cannot handle (causes word2ph mismatch).
fn normalize_text_for_tts(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\u{201C}' | '\u{201D}' => '"',  // "" → "
            '\u{2018}' | '\u{2019}' => '\'', // '' → '
            '\u{2014}' => ',',                // — em dash → comma
            '\u{2026}' => '.',                // … ellipsis → period
            '\u{3000}' => ' ',                // ideographic space → space
            '\u{00A0}' => ' ',                // non-breaking space → space
            _ => c,
        })
        .collect()
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting dora-gpt-sovits-mlx TTS node...");

    let config = Config::from_env();
    if let Some(ref voice) = config.voice_character {
        log::info!("Using voice character: {}", voice);
    }
    log::info!("Configuration: {:?}", config);

    // Create Dora node (supports --name <id> for dynamic mode)
    let args: Vec<String> = std::env::args().collect();
    let node_id = if args.len() > 2 && args[1] == "--name" {
        Some(args[2].clone())
    } else {
        None
    };

    let (mut node, events) = if let Some(id) = node_id {
        log::info!("Initializing as dynamic node: {}", id);
        DoraNode::init_from_node_id(NodeId::from(id))?
    } else {
        DoraNode::init_from_env()?
    };

    // Initialize TTS engine
    log::info!("Initializing GPT-SoVITS engine...");
    let start = Instant::now();

    let cloner_config = VoiceClonerConfig {
        t2s_weights: config.t2s_weights.clone(),
        bert_weights: config.bert_weights.clone(),
        bert_tokenizer: config.bert_tokenizer.clone(),
        vits_weights: config.vits_weights.clone(),
        vits_pretrained_base: config.vits_pretrained_base.clone(),
        hubert_weights: config.hubert_weights.clone(),
        vits_onnx_path: config.vits_onnx_path.clone(),
        use_mlx_vits: config.use_mlx_vits,
        use_gpu_mel: true,  // Use GPU-accelerated FFT for mel loading
        sample_rate: config.sample_rate,
        top_k: config.top_k,
        top_p: config.top_p,
        temperature: config.temperature,
        repetition_penalty: config.repetition_penalty,
        noise_scale: config.noise_scale,
        speed: config.speed,
    };

    // Prepare synthesis options with optional timeout
    let synthesis_timeout = config.synthesis_timeout_secs.map(Duration::from_secs);

    let mut cloner = match VoiceCloner::new(cloner_config) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to initialize VoiceCloner: {}", e);
            send_log(&mut node, "ERROR", &format!("Engine init failed: {}", e))?;
            return Err(e.into());
        }
    };

    // Set reference audio
    // Priority: codes_path (precomputed) > ref_text (few-shot) > zero-shot
    if let (Some(ref codes_path), Some(ref ref_text)) = (&config.codes_path, &config.ref_text) {
        log::info!(
            "Setting reference with precomputed codes (few-shot mode): audio={}, codes={}",
            config.ref_audio,
            codes_path
        );
        if let Err(e) = cloner.set_reference_with_precomputed_codes(&config.ref_audio, ref_text, codes_path) {
            log::error!("Failed to set reference with codes: {}", e);
            send_log(
                &mut node,
                "ERROR",
                &format!("Reference setup failed: {}", e),
            )?;
            return Err(e.into());
        }
    } else if let Some(ref ref_text) = config.ref_text {
        log::info!(
            "Setting reference audio with text (few-shot mode): {}",
            config.ref_audio
        );
        if let Err(e) = cloner.set_reference_audio_with_text(&config.ref_audio, ref_text) {
            log::error!("Failed to set reference audio: {}", e);
            send_log(
                &mut node,
                "ERROR",
                &format!("Reference audio setup failed: {}", e),
            )?;
            return Err(e.into());
        }
    } else {
        log::info!(
            "Setting reference audio (zero-shot mode): {}",
            config.ref_audio
        );
        if let Err(e) = cloner.set_reference_audio(&config.ref_audio) {
            log::error!("Failed to set reference audio: {}", e);
            send_log(
                &mut node,
                "ERROR",
                &format!("Reference audio setup failed: {}", e),
            )?;
            return Err(e.into());
        }
    }

    log::info!(
        "GPT-SoVITS engine initialized in {:.2}s (few_shot={})",
        start.elapsed().as_secs_f32(),
        cloner.is_few_shot_mode()
    );
    send_log(
        &mut node,
        "INFO",
        &format!(
            "TTS ready: few_shot={}, sample_rate={}Hz",
            cloner.is_few_shot_mode(),
            config.sample_rate
        ),
    )?;

    // Statistics
    let mut total_segments: u64 = 0;
    let mut total_audio_duration: f64 = 0.0;
    let mut total_processing_time: f64 = 0.0;

    // Event loop
    let events = futures::executor::block_on_stream(events);

    for event in events {
        match event {
            Event::Input { id, data, metadata } => {
                let input_id = id.as_str();

                if input_id == "text" {
                    // Extract text from Arrow string array
                    let text = match data.as_any().downcast_ref::<StringArray>() {
                        Some(arr) if arr.len() > 0 => arr.value(0).to_string(),
                        _ => {
                            log::warn!("Received empty or invalid text input");
                            send_segment_complete(&mut node, "empty", &metadata.parameters)?;
                            continue;
                        }
                    };

                    if text.trim().is_empty() {
                        log::debug!("Skipping empty text");
                        send_segment_complete(&mut node, "empty", &metadata.parameters)?;
                        continue;
                    }

                    // Check if text is only punctuation/whitespace (match dora-primespeech behavior)
                    let stripped = text.trim().replace(|c: char| {
                        c.is_whitespace() || matches!(c,
                            '\u{3002}' | '\u{FF01}' | '\u{FF1F}' | '\u{FF0C}' | '\u{3001}' | '\u{FF1B}' | '\u{FF1A}' |
                            '\u{201C}' | '\u{201D}' | '\u{2018}' | '\u{2019}' | '\u{FF08}' | '\u{FF09}' | '\u{3010}' | '\u{3011}' | '\u{300A}' | '\u{300B}' |
                            '.' | '!' | '?' | ',' | ';' | ':' | '"' | '\'' | '(' | ')' | '[' | ']'
                        )
                    }, "");
                    if stripped.is_empty() {
                        log::debug!("Skipping punctuation-only text: \"{}\"", truncate_text(&text, 30));
                        send_segment_complete(&mut node, "skipped", &metadata.parameters)?;
                        continue;
                    }

                    // Extract metadata
                    let question_id = get_string_param(&metadata.parameters, "question_id")
                        .unwrap_or_else(|| "default".to_string());
                    let session_status = get_string_param(&metadata.parameters, "session_status")
                        .unwrap_or_else(|| "unknown".to_string());

                    // Normalize text to avoid G2P failures (curly quotes, etc.)
                    let text = normalize_text_for_tts(&text);

                    log::info!(
                        "Synthesizing: \"{}\" (question_id={}, len={})",
                        truncate_text(&text, 50),
                        question_id,
                        text.len()
                    );

                    let synth_start = Instant::now();
                    let mut had_error = false;
                    let mut last_error: Option<String> = None;

                    if is_ssml(&text) {
                        // --- SSML path ---
                        let ssml_segments = match parse_ssml(&text) {
                            Ok(segs) => segs,
                            Err(e) => {
                                log::warn!("SSML parse error: {}, falling back to plain text", e);
                                vec![SsmlSegment::Text {
                                    text: ssml::strip_xml_tags(&text),
                                    speed: 1.0,
                                }]
                            }
                        };

                        let total_segs = ssml_segments.len();
                        log::info!("SSML mode: {} segments", total_segs);

                        let mut audio_idx: i64 = 0;

                        for (idx, segment) in ssml_segments.iter().enumerate() {
                            let is_final = idx == total_segs - 1;

                            match segment {
                                SsmlSegment::Text { text: seg_text, speed } => {
                                    let seg_start = Instant::now();

                                    log::debug!(
                                        "SSML segment {}/{}: text=\"{}\" speed={:.2}",
                                        idx + 1, total_segs,
                                        truncate_text(seg_text, 30), speed
                                    );

                                    let mut options = SynthesisOptions::default();
                                    if let Some(timeout) = synthesis_timeout {
                                        options.timeout = Some(timeout);
                                    }
                                    if (*speed - 1.0).abs() > f32::EPSILON {
                                        options.speed_override = Some(*speed);
                                    }

                                    match cloner.synthesize_with_options(seg_text, options) {
                                        Ok(audio) => {
                                            let elapsed = seg_start.elapsed().as_secs_f32();
                                            let rtf = elapsed / audio.duration;

                                            total_segments += 1;
                                            total_audio_duration += audio.duration as f64;
                                            total_processing_time += elapsed as f64;

                                            log::info!(
                                                "SSML segment {}/{}: {:.2}s audio in {:.2}s (RTF={:.2}x, speed={:.2})",
                                                idx + 1, total_segs,
                                                audio.duration, elapsed, rtf, speed
                                            );

                                            let audio_array = Float32Array::from(audio.samples.clone());
                                            let mut out_meta = BTreeMap::new();
                                            out_meta.insert("question_id".to_string(), Parameter::String(question_id.clone()));
                                            out_meta.insert("session_status".to_string(), Parameter::String(session_status.clone()));
                                            out_meta.insert("sample_rate".to_string(), Parameter::Integer(audio.sample_rate as i64));
                                            out_meta.insert("duration".to_string(), Parameter::String(format!("{:.3}", audio.duration)));
                                            out_meta.insert("is_final".to_string(), Parameter::String(is_final.to_string()));
                                            out_meta.insert("fragment_index".to_string(), Parameter::Integer(audio_idx));
                                            out_meta.insert("fragment_total".to_string(), Parameter::Integer(total_segs as i64));

                                            node.send_output("audio".into(), out_meta, audio_array)?;
                                            audio_idx += 1;
                                        }
                                        Err(e) => {
                                            log::error!("SSML synthesis failed for segment {}: {}", idx + 1, e);
                                            had_error = true;
                                            last_error = Some(e.to_string());
                                        }
                                    }
                                }
                                SsmlSegment::Silence { duration_ms } => {
                                    let silence_samples = (config.sample_rate as f32 * *duration_ms as f32 / 1000.0) as usize;
                                    let silence_duration = *duration_ms as f32 / 1000.0;

                                    log::debug!(
                                        "SSML segment {}/{}: silence {:.1}ms",
                                        idx + 1, total_segs, duration_ms
                                    );

                                    let audio_array = Float32Array::from(vec![0.0f32; silence_samples]);
                                    let mut out_meta = BTreeMap::new();
                                    out_meta.insert("question_id".to_string(), Parameter::String(question_id.clone()));
                                    out_meta.insert("session_status".to_string(), Parameter::String(session_status.clone()));
                                    out_meta.insert("sample_rate".to_string(), Parameter::Integer(config.sample_rate as i64));
                                    out_meta.insert("duration".to_string(), Parameter::String(format!("{:.3}", silence_duration)));
                                    out_meta.insert("is_final".to_string(), Parameter::String(is_final.to_string()));
                                    out_meta.insert("is_silence".to_string(), Parameter::String("true".to_string()));
                                    out_meta.insert("fragment_index".to_string(), Parameter::Integer(audio_idx));
                                    out_meta.insert("fragment_total".to_string(), Parameter::Integer(total_segs as i64));

                                    node.send_output("audio".into(), out_meta, audio_array)?;
                                    audio_idx += 1;
                                }
                            }
                        }
                    } else {
                        // --- Plain text path ---
                        let sentences = if config.return_fragment {
                            split_into_sentences(&text, config.fragment_min_chars)
                        } else {
                            vec![text.clone()]
                        };

                        let total_sentences = sentences.len();
                        if config.return_fragment && total_sentences > 1 {
                            log::info!(
                                "Streaming mode: split into {} sentences",
                                total_sentences
                            );
                        }

                        for (idx, sentence) in sentences.iter().enumerate() {
                            let is_final = idx == total_sentences - 1;
                            let sentence_start = Instant::now();

                            if config.return_fragment {
                                log::debug!(
                                    "Processing sentence {}/{}: \"{}\"",
                                    idx + 1,
                                    total_sentences,
                                    truncate_text(sentence, 30)
                                );
                            }

                            let synth_result = if let Some(timeout) = synthesis_timeout {
                                let options = SynthesisOptions::with_timeout(timeout);
                                cloner.synthesize_with_options(sentence, options)
                            } else {
                                cloner.synthesize(sentence)
                            };

                            match synth_result {
                                Ok(audio) => {
                                    let elapsed = sentence_start.elapsed().as_secs_f32();
                                    let rtf = elapsed / audio.duration;

                                    total_segments += 1;
                                    total_audio_duration += audio.duration as f64;
                                    total_processing_time += elapsed as f64;

                                    if config.return_fragment {
                                        log::info!(
                                            "Sentence {}/{}: {:.2}s audio in {:.2}s (RTF={:.2}x)",
                                            idx + 1, total_sentences,
                                            audio.duration, elapsed, rtf
                                        );
                                    } else {
                                        log::info!(
                                            "Synthesis complete: {:.2}s audio in {:.2}s (RTF={:.2}x)",
                                            audio.duration, elapsed, rtf
                                        );
                                    }

                                    let mut samples = audio.samples.clone();
                                    let mut output_duration = audio.duration;

                                    if config.return_fragment && !is_final && config.fragment_interval > 0.0 {
                                        let silence_samples = (audio.sample_rate as f32 * config.fragment_interval) as usize;
                                        samples.extend(vec![0.0f32; silence_samples]);
                                        output_duration += config.fragment_interval;
                                    }

                                    let audio_array = Float32Array::from(samples);
                                    let mut out_meta = BTreeMap::new();
                                    out_meta.insert("question_id".to_string(), Parameter::String(question_id.clone()));
                                    out_meta.insert("session_status".to_string(), Parameter::String(session_status.clone()));
                                    out_meta.insert("sample_rate".to_string(), Parameter::Integer(audio.sample_rate as i64));
                                    out_meta.insert("duration".to_string(), Parameter::String(format!("{:.3}", output_duration)));
                                    out_meta.insert("is_final".to_string(), Parameter::String(is_final.to_string()));
                                    out_meta.insert("fragment_index".to_string(), Parameter::Integer(idx as i64));
                                    out_meta.insert("fragment_total".to_string(), Parameter::Integer(total_sentences as i64));

                                    node.send_output("audio".into(), out_meta, audio_array)?;
                                }
                                Err(e) => {
                                    log::error!("Synthesis failed for sentence {}: {}", idx + 1, e);
                                    had_error = true;
                                    last_error = Some(e.to_string());
                                    if !config.return_fragment {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    // Send completion signal
                    if had_error {
                        send_log(
                            &mut node,
                            "ERROR",
                            &format!(
                                "Synthesis failed for '{}': {}",
                                truncate_text(&text, 30),
                                last_error.as_deref().unwrap_or("unknown error")
                            ),
                        )?;

                        // CRITICAL: Send a tiny silent audio chunk on error so the
                        // audio player still emits audio_complete. Without this,
                        // the text segmenter's flow control gets permanently stuck
                        // (is_sending stays True, no more segments sent).
                        let silence_samples = (config.sample_rate as f32 * 0.05) as usize; // 50ms
                        let silence_audio = Float32Array::from(vec![0.0f32; silence_samples]);
                        let mut silence_meta = BTreeMap::new();
                        silence_meta.insert("question_id".to_string(), Parameter::String(question_id.clone()));
                        silence_meta.insert("session_status".to_string(), Parameter::String(session_status.clone()));
                        silence_meta.insert("sample_rate".to_string(), Parameter::Integer(config.sample_rate as i64));
                        silence_meta.insert("duration".to_string(), Parameter::String("0.050".to_string()));
                        silence_meta.insert("is_final".to_string(), Parameter::String("true".to_string()));
                        silence_meta.insert("is_error_placeholder".to_string(), Parameter::String("true".to_string()));
                        node.send_output("audio".into(), silence_meta, silence_audio)?;
                        log::info!("Sent silent audio placeholder for failed segment (keeps pipeline flowing)");

                        let mut meta = BTreeMap::new();
                        meta.insert("question_id".to_string(), Parameter::String(question_id));
                        meta.insert("session_status".to_string(), Parameter::String("error".to_string()));
                        meta.insert("error".to_string(), Parameter::String(last_error.unwrap_or_else(|| "unknown".to_string())));
                        meta.insert("error_stage".to_string(), Parameter::String("synthesis".to_string()));

                        let arr = StringArray::from(vec!["error"]);
                        node.send_output("segment_complete".into(), meta, arr)?;
                    } else {
                        let total_elapsed = synth_start.elapsed().as_secs_f32();
                        log::debug!("Total synthesis time: {:.2}s", total_elapsed);

                        send_segment_complete_with_meta(
                            &mut node,
                            "completed",
                            &question_id,
                            &session_status,
                        )?;
                    }
                } else if input_id == "control" {
                    // Handle control commands
                    let command = match data.as_any().downcast_ref::<StringArray>() {
                        Some(arr) if arr.len() > 0 => arr.value(0).to_string(),
                        _ => continue,
                    };

                    match command.as_str() {
                        "stats" => {
                            let avg_rtf = if total_audio_duration > 0.0 {
                                total_processing_time / total_audio_duration
                            } else {
                                0.0
                            };
                            log::info!(
                                "Stats: {} segments, {:.1}s audio, avg RTF={:.2}x",
                                total_segments,
                                total_audio_duration,
                                avg_rtf
                            );
                        }
                        "reset" => {
                            total_segments = 0;
                            total_audio_duration = 0.0;
                            total_processing_time = 0.0;
                            log::info!("Statistics reset");
                        }
                        _ => {
                            log::warn!("Unknown control command: {}", command);
                        }
                    }
                }
            }
            Event::Stop { .. } => {
                log::info!("Received stop signal, shutting down...");
                break;
            }
            _ => {}
        }
    }

    log::info!(
        "TTS node stopped. Total: {} segments, {:.1}s audio",
        total_segments,
        total_audio_duration
    );
    Ok(())
}

fn get_string_param(params: &BTreeMap<String, Parameter>, key: &str) -> Option<String> {
    params.get(key).and_then(|p| match p {
        Parameter::String(s) => Some(s.clone()),
        Parameter::Integer(i) => Some(i.to_string()),
        _ => None,
    })
}

fn send_log(node: &mut DoraNode, level: &str, message: &str) -> Result<()> {
    let log_entry = serde_json::json!({
        "level": level,
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "node": "gpt-sovits-mlx"
    });

    let arr = StringArray::from(vec![log_entry.to_string()]);
    node.send_output("log".into(), BTreeMap::new(), arr)?;
    Ok(())
}

fn send_segment_complete(
    node: &mut DoraNode,
    status: &str,
    params: &BTreeMap<String, Parameter>,
) -> Result<()> {
    let question_id = get_string_param(params, "question_id").unwrap_or_else(|| "default".to_string());
    let session_status =
        get_string_param(params, "session_status").unwrap_or_else(|| "unknown".to_string());
    send_segment_complete_with_meta(node, status, &question_id, &session_status)
}

fn send_segment_complete_with_meta(
    node: &mut DoraNode,
    status: &str,
    question_id: &str,
    session_status: &str,
) -> Result<()> {
    let mut meta = BTreeMap::new();
    meta.insert(
        "question_id".to_string(),
        Parameter::String(question_id.to_string()),
    );
    meta.insert(
        "session_status".to_string(),
        Parameter::String(session_status.to_string()),
    );

    let arr = StringArray::from(vec![status]);
    node.send_output("segment_complete".into(), meta, arr)?;
    Ok(())
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max_len).collect();
        format!("{}...", truncated)
    }
}
