//! Logging initialization module

use log::SetLoggerError;
use simplelog::*;
use std::str::FromStr;

/// Logging struct
#[derive(Debug)]
pub struct Logging {}

/// Logging config struct
#[derive(Debug)]
pub struct LoggingConfig {
    time_format: &'static str,
    level: LevelFilter,
}

impl Logging {
    /// Creates config from environment variables
    pub fn from_env() -> LoggingConfig {
        let time_format = if dotenv::var("LOG_SHOW_DATETIME")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false)
        {
            "%Y-%m-%d %H:%M:%S"
        } else {
            ""
        };

        let level = LevelFilter::from_str(dotenv::var("LOG_LEVEL").unwrap().as_str()).unwrap();

        LoggingConfig { time_format, level }
    }
}

impl LoggingConfig {
    /// Initializes logging
    pub fn init(&self) -> Result<(), SetLoggerError> {
        let config = ConfigBuilder::new()
            .set_time_format_str(self.time_format)
            .set_target_level(LevelFilter::Debug)
            .build();

        TermLogger::init(self.level, config, TerminalMode::Mixed, ColorChoice::Auto)
    }
}
