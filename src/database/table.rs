//! Creating Table instance.

use std::path::PathBuf;

use crate::database::Database;

pub mod create;
pub mod exists;
pub mod read;
pub mod write;


#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub file: PathBuf,
}

impl Database {
    /// Creates a new table in the database
    pub fn table(&self, name: &str) -> Table {
        let mut file = PathBuf::from(self.path.as_path());
        file.push(&name);

        Table {
            name: name.to_string(),
            file,
        }
    }
}
