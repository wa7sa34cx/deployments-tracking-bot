//! Database module.

use std::path::PathBuf;

pub mod init;
pub mod create_table;

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
}
