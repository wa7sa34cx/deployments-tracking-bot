//! Database module.

use std::path::PathBuf;

pub mod init;
pub mod table;
pub mod table_create;
pub mod table_exists;
pub mod table_read;
pub mod table_write;

/// Main database struct
#[derive(Debug)]
pub struct Database {
    path: PathBuf,
}
