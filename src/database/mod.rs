//! Database module.

use std::path::PathBuf;

pub mod init;

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
}
