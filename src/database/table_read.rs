//! Creating table.

use std::io;
use tokio::fs;

use crate::database::table::Table;

impl Table {
    /// Reads data from the table
    pub async fn read(&self) -> Result<Vec<String>, io::Error> {
        // Read from the file
        let contents = fs::read_to_string(&self.file).await?;

        let lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

        log::debug!("data from table {} has been successfully read", self.name);

        Ok(lines)
    }
}
