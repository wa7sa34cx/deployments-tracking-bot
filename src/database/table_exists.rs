//! Checking if the table exists.

use crate::database::table::Table;

impl Table {
    /// Checks if the table exists
    pub fn exists(&self) -> bool {
        self.file.exists()
    }
}
