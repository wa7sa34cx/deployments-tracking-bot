//! Creating table.
 
use tokio::fs::File;
use std::io;

use crate::database::table::Table;

impl Table {
    /// Creates a new table in the database
    pub async fn create(&self) -> Result<(), io::Error> {
        File::create(&self.file).await?;

        log::info!("table {} has been successfully created", self.name);

        Ok(())
    }
}
