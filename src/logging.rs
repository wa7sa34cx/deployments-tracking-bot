//! Logging initialization module

use simplelog::*;

pub fn init() {
    let time_format = if dotenv::var("LOG_SHOW_DATETIME")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false)
    {
        "%Y-%m-%d %H:%M:%S"
    } else {
        ""
    };

    let config = ConfigBuilder::new()
        .set_time_format_str(time_format)
        .set_target_level(LevelFilter::Info)
        .build();

    TermLogger::init(
        LevelFilter::Info,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}
