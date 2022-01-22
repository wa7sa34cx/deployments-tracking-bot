//! Logging initialization module

use simplelog::*;

pub fn init() {
    let config = ConfigBuilder::new()
        .set_time_format_str("")
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
