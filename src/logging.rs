//! Logging initialization module

use simplelog::*;
use std::str::FromStr;

/// Logging struct
#[derive(Debug)]
pub struct Logging {
    time_format: &'static str,
    level: LevelFilter,
}

impl Logging {
    /// Creates config from environment variables
    pub fn from_env() -> Self {
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

        Self { time_format, level }
    }

    /// Initializes logging
    pub fn init(&self) {
        let config = ConfigBuilder::new()
            .set_time_format_str(self.time_format)
            .set_target_level(LevelFilter::Error)
            .build();

        TermLogger::init(
            self.level,
            config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .unwrap();
    }
}
