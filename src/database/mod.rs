//! Database module.

pub mod init;

use std::path::PathBuf;
use tokio::fs;

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
}
