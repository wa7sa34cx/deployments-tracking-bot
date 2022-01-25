//! Database module.

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

pub mod init;
pub mod table;

/// Main database struct
pub struct Database(Arc<SharedDatabase>);

impl Clone for Database {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for Database {
    type Target = SharedDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Internal struct for safe multithreading
#[derive(Debug)]
pub struct SharedDatabase {
    path: PathBuf,
}
