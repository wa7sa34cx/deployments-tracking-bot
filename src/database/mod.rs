//! Database module.

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

pub mod init;
pub mod table;

/// Main Database struct
#[derive(Debug)]
pub struct Database(Arc<DatabaseConfig>);

/// Database configuration
#[derive(Debug)]
pub struct DatabaseConfig {
    path: PathBuf,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for Database {
    type Target = DatabaseConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    /// Creates DatabaseConfig from environment
    ///
    /// # Panics
    ///
    /// Panics if the DB_PATH variable are not specified in environment
    pub fn from_env() -> DatabaseConfig {
        let path = PathBuf::from(dotenv::var("DB_PATH").unwrap());

        DatabaseConfig { path }
    }
}
