//! Dora TTS Node using MLX-based GPT-SoVITS
//!
//! This node provides text-to-speech synthesis optimized for Apple Silicon
//! using the GPT-SoVITS model via MLX (Metal acceleration).
//!
//! Interface compatible with dora-primespeech:
//! - Input `text`: Arrow string array with metadata (question_id, session_status)
//! - Output `audio`: Arrow float32 array with metadata (sample_rate, duration, question_id)
//! - Output `segment_complete`: Arrow string ("completed"/"error") with metadata

use dora_node_api::{
    arrow::array::{Array, Float32Array, StringArray},
    dora_core::config::NodeId,
    DoraNode, Event, Parameter,
};
use eyre::Result;
use gpt_sovits_mlx::voice_clone::{VoiceCloner, VoiceClonerConfig, SynthesisOptions};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

mod config;
use config::Config;

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
                            continue;
                        }
                    };

                    if text.trim().is_empty() {
                        log::debug!("Skipping empty text");
                        send_segment_complete(&mut node, "empty", &metadata.parameters)?;
                        continue;
                    }

                    // Extract metadata
                    let question_id = get_string_param(&metadata.parameters, "question_id")
                        .unwrap_or_else(|| "default".to_string());
                    let session_status = get_string_param(&metadata.parameters, "session_status")
                        .unwrap_or_else(|| "unknown".to_string());

                    log::info!(
                        "Synthesizing: \"{}\" (question_id={}, len={})",
                        truncate_text(&text, 50),
                        question_id,
                        text.len()
                    );

                    let synth_start = Instant::now();

                    // Synthesize (with optional timeout)
                    let synth_result = if let Some(timeout) = synthesis_timeout {
                        let options = SynthesisOptions::with_timeout(timeout);
                        cloner.synthesize_with_options(&text, options)
                    } else {
                        cloner.synthesize(&text)
                    };

                    match synth_result {
                        Ok(audio) => {
                            let elapsed = synth_start.elapsed().as_secs_f32();
                            let rtf = elapsed / audio.duration;

                            total_segments += 1;
                            total_audio_duration += audio.duration as f64;
                            total_processing_time += elapsed as f64;

                            log::info!(
                                "Synthesis complete: {:.2}s audio in {:.2}s (RTF={:.2}x)",
                                audio.duration,
                                elapsed,
                                rtf
                            );

                            // Send audio output
                            let audio_array = Float32Array::from(audio.samples.clone());
                            let mut out_meta = BTreeMap::new();
                            out_meta.insert(
                                "question_id".to_string(),
                                Parameter::String(question_id.clone()),
                            );
                            out_meta.insert(
                                "session_status".to_string(),
                                Parameter::String(session_status.clone()),
                            );
                            out_meta.insert(
                                "sample_rate".to_string(),
                                Parameter::Integer(audio.sample_rate as i64),
                            );
                            out_meta.insert(
                                "duration".to_string(),
                                Parameter::String(format!("{:.3}", audio.duration)),
                            );

                            node.send_output(
                                "audio".into(),
                                out_meta.clone(),
                                audio_array,
                            )?;

                            // Send completion signal
                            send_segment_complete_with_meta(
                                &mut node,
                                "completed",
                                &question_id,
                                &session_status,
                            )?;
                        }
                        Err(e) => {
                            log::error!("Synthesis failed: {}", e);
                            send_log(
                                &mut node,
                                "ERROR",
                                &format!("Synthesis failed for '{}': {}", truncate_text(&text, 30), e),
                            )?;

                            // Send error completion
                            let mut meta = BTreeMap::new();
                            meta.insert(
                                "question_id".to_string(),
                                Parameter::String(question_id),
                            );
                            meta.insert(
                                "session_status".to_string(),
                                Parameter::String("error".to_string()),
                            );
                            meta.insert("error".to_string(), Parameter::String(e.to_string()));
                            meta.insert(
                                "error_stage".to_string(),
                                Parameter::String("synthesis".to_string()),
                            );

                            let arr = StringArray::from(vec!["error"]);
                            node.send_output("segment_complete".into(), meta, arr)?;
                        }
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
