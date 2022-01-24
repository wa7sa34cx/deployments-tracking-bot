//! Logging initialization module

use simplelog::*;
use std::str::FromStr;

// Config for creating... config
#[derive(Debug)]
struct Config {
    time_format: &'static str,
    level: LevelFilter,
}

impl Config {
    // Creates config from environment variables
    fn from_env() -> Self {
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
}

/// Initializes logging
pub fn init() {
    let config = Config::from_env();

    let log_config = ConfigBuilder::new()
        .set_time_format_str(config.time_format)
        .set_target_level(config.level)
        .build();

    TermLogger::init(
        LevelFilter::Info,
        log_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}
