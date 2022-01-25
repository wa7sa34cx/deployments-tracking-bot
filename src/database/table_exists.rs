//! Checking if the table exists.

use crate::database::table::Table;

impl Table {
    /// Checks if the table exists
    pub fn exists(&self) -> bool {
        let exists = self.file.exists();

        if exists {
            log::debug!("table {} exists", self.name);
        } else {
            log::debug!("table {} doesn't exist", self.name);
        }

        exists
    }
}
