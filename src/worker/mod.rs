//! Worker module

use crate::database::Database;
use crate::digitalocean::DigitalOcean;
use crate::telegram::Telegram;

pub mod init;
pub mod work;

/// Main Worker struct
/// Cloning is cheap
#[derive(Debug, Clone)]
pub struct Worker {
    pub digitalocean: DigitalOcean,
    pub database: Database,
    pub telegram: Telegram,
    pub config: WorkerConfig,
}

/// Worker configuration
/// Cloning is cheap
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub interval: u64,
}

impl Worker {
    /// Creates Worker from environment
    ///
    /// # Panics
    ///
    /// Panics if the WORK_CHECK_INTERVAL variable are not specified in environment
    pub fn from_env() -> WorkerConfig {
        let interval = dotenv::var("WORK_INTERVAL")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        WorkerConfig { interval }
    }
}
