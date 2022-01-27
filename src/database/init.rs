//! Database initialization.

use std::io;
use std::sync::Arc;
use tokio::fs;

use crate::database::{Database, DatabaseConfig};

impl DatabaseConfig {
    /// Initializes database
    pub async fn init(self) -> Result<Database, io::Error> {
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

        Ok(Database(Arc::new(DatabaseConfig { path: self.path })))
    }
}
