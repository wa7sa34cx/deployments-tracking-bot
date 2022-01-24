//! Creating table.
 
use tokio::fs;
use std::io;

use crate::database::table::Table;

impl Table {
    /// Reads data from the table
    pub async fn read(&self) -> Result<Vec<String>, io::Error> {
        let contents = fs::read_to_string(&self.file).await?;
        let lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

        Ok(lines)
    }
}
