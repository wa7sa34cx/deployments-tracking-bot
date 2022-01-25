//! Creating table

use std::io;
use tokio::fs;

use crate::database::table::Table;

impl Table {
    // Writes data to the table
    pub async fn write(&self, lines: Vec<&str>) -> Result<(), io::Error> {
        let contents = lines.join("\n");

        // Write to the file
        fs::write(&self.file, &contents).await?;

        log::debug!("data to table {} has been successfully written", self.name);

        Ok(())
    }
}
