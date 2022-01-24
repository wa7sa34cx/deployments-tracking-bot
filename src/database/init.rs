//! Database initialization

use std::path::PathBuf;
use tokio::fs;

use crate::database::Database;

impl Database {
    // Creates new Db instance
    pub async fn init() -> anyhow::Result<Self> {
        let path = PathBuf::from(dotenv::var("DB_PATH").unwrap());

        // Remove the old DB and create an empty new one
        fs::remove_dir_all(&path).await?;
        fs::create_dir(&path).await?;

        Ok(Self {
            path,
        })
    }
}
