//! Database initialization

use std::path::PathBuf;
use std::io;
use tokio::fs;

use crate::database::Database;

impl Database {
    // Creates new Db instance
    pub async fn init() -> Result<Self, io::Error> {
        let path = PathBuf::from(dotenv::var("DB_PATH").unwrap());

        // Remove the old database
        match fs::remove_dir_all(&path).await {
            Ok(_) => {
                log::warn!("the old database has been deleted, creating...");
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    log::warn!("the database does not exist, creating...");
                } else {
                    return Err(e);
                }
            },
        };

        // And create an empty new one
        fs::create_dir(&path).await?;
        log::info!("the database has been created");

        Ok(Self {
            path,
        })
    }
}
