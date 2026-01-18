//! Command-line interface for MoFA Studio
//!
//! Provides CLI argument parsing for configuring the application at startup.
//!
//! # Usage
//!
//! ```bash
//! # Show help
//! mofa-studio --help
//!
//! # Start with dark mode
//! mofa-studio --dark-mode
//!
//! # Set log level
//! mofa-studio --log-level debug
//!
//! # Specify custom dataflow
//! mofa-studio --dataflow /path/to/voice-chat.yml
//!
//! # Custom audio sample rate
//! mofa-studio --sample-rate 44100
//! ```

use clap::Parser;

/// MoFA Studio - AI-powered voice chat desktop application
///
/// A GPU-accelerated desktop UI for real-time multi-participant voice
/// conversations with LLM integration, built with Rust and Makepad.
#[derive(Parser, Debug, Clone)]
#[command(name = "mofa-studio")]
#[command(author = "MoFA Team")]
#[command(version)]
#[command(about = "AI-powered voice chat desktop application", long_about = None)]
pub struct Args {
    /// Path to dataflow YAML file
    ///
    /// Specifies the Dora dataflow configuration to use for the voice chat.
    /// If not provided, uses the default voice-chat.yml from the app directory.
    #[arg(short, long, value_name = "FILE")]
    pub dataflow: Option<String>,

    /// Audio sample rate in Hz
    ///
    /// Sample rate for audio input/output. Common values: 16000, 32000, 44100, 48000.
    /// Default is 32000 Hz which is optimal for voice.
    #[arg(long, default_value = "32000", value_name = "HZ")]
    pub sample_rate: u32,

    /// Start in dark mode
    ///
    /// When set, the application starts with dark mode enabled.
    /// This can also be toggled from within the application.
    #[arg(long)]
    pub dark_mode: bool,

    /// Log level for output
    ///
    /// Controls the verbosity of log output. Available levels:
    /// error, warn, info, debug, trace
    #[arg(long, default_value = "info", value_name = "LEVEL")]
    pub log_level: String,

    /// Window width in pixels
    #[arg(long, default_value = "1400", value_name = "PIXELS")]
    pub width: u32,

    /// Window height in pixels
    #[arg(long, default_value = "900", value_name = "PIXELS")]
    pub height: u32,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            dataflow: None,
            sample_rate: 32000,
            dark_mode: false,
            log_level: "info".to_string(),
            width: 1400,
            height: 900,
        }
    }
}

impl Args {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get log level as env_logger filter string
    pub fn log_filter(&self) -> &str {
        match self.log_level.to_lowercase().as_str() {
            "error" => "error",
            "warn" | "warning" => "warn",
            "info" => "info",
            "debug" => "debug",
            "trace" => "trace",
            _ => "info",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_args() {
        let args = Args::default();
        assert_eq!(args.sample_rate, 32000);
        assert!(!args.dark_mode);
        assert_eq!(args.log_level, "info");
        assert_eq!(args.width, 1400);
        assert_eq!(args.height, 900);
    }

    #[test]
    fn test_log_filter() {
        let mut args = Args::default();

        args.log_level = "debug".to_string();
        assert_eq!(args.log_filter(), "debug");

        args.log_level = "WARNING".to_string();
        assert_eq!(args.log_filter(), "warn");

        args.log_level = "invalid".to_string();
        assert_eq!(args.log_filter(), "info");
    }
}
