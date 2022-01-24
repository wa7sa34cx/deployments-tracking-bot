//! Database initialization

use std::io;
use std::path::PathBuf;
use tokio::fs;

use crate::database::Database;

impl Database {
    /// Creates new Db instance
    ///
    /// # Panics
    ///
    /// Panics if the DB_PATH variable are not specified in environment
    pub async fn init() -> Result<Self, io::Error> {
        let path = PathBuf::from(dotenv::var("DB_PATH").unwrap());

        // Remove the old database
        match fs::remove_dir_all(&path).await {
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
        fs::create_dir(&path).await?;
        log::debug!("the database has been successfully created");

        Ok(Self { path })
    }
}
