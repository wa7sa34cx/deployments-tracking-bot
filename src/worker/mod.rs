//! Worker module

pub mod init;
pub mod work;

/// Worker
pub struct Worker {
    rate_limit: u64,
}

impl Worker {
    /// Creates DatabaseConfig from environment
    ///
    /// # Panics
    ///
    /// Panics if the DB_PATH variable are not specified in environment
    pub fn from_env() -> WorkerConfig {
        let rate_limit = PathBuf::from(dotenv::var("DB_PATH").unwrap());

        WorkerConfig { path }
    }
}