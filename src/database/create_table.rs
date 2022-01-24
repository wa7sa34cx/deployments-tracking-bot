//! Creating table
 
use tokio::fs::File;
use std::io;
use std::path::PathBuf;

use crate::database::Database;
use crate::digitalocean::apps::App;

impl Database {
    // Creates a new table in the database
    pub async fn create_table(&self, app: &App) -> Result<(), io::Error> {
        let mut file = PathBuf::from(self.path.as_path());
        // Create full path to file
        file.push(&app.id);

        File::create(file).await?;

        log::info!("table {} has been successfully created", app.id);

        Ok(())
    }
}
