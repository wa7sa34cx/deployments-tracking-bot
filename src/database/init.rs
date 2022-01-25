//! Database initialization

use std::io;
use std::path::PathBuf;
use tokio::fs;

use crate::database::Database;

impl Database {
    /// Creates new Database instance
    ///
    /// # Panics
    ///
    /// Panics if the DB_PATH variable are not specified in environment
    pub fn from_env() -> Self {
        let path = PathBuf::from(dotenv::var("DB_PATH").unwrap());

        Self { path }
    }

    // Initializes database
    pub async fn init(self) -> Result<Self, io::Error> {
        // Remove the old database
        match fs::remove_dir_all(&self.path).await {
            Ok(_) => {
                log::debug!("the old database has been deleted, creating...");
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    log::debug!("the database does not exist, creating...");
                } else {
                    return Err(e);
                }
            }
        };

        // And create an empty new one
        fs::create_dir(&self.path).await?;
        log::debug!("the database has been successfully initialized");

        Ok(self)
    }
}
