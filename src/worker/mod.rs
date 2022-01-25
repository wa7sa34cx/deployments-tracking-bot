//! Worker module

use crate::database::Database;
use crate::digitalocean::DigitalOcean;

pub mod init;
pub mod work;

/// Worker
pub struct Worker {
    digitalocean: DigitalOcean,
    database: Database,
    config: WorkerConfig,
}

/// Worker configuration
pub struct WorkerConfig {
    rate_limit: u64,
}

impl Worker {
    /// Creates Worker from environment
    ///
    /// # Panics
    ///
    /// Panics if the DO_RATE_LIMIT variable are not specified in environment
    pub fn from_env() -> WorkerConfig {
        let rate_limit = dotenv::var("DO_RATE_LIMIT").unwrap().parse::<u64>().unwrap();

        WorkerConfig { rate_limit }
    }
}