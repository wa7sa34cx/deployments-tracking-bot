//! DigitalOcean Module.

use reqwest::Client;
use std::ops::Deref;
use std::sync::Arc;

pub mod apps;
pub mod deployments;
pub mod error;
pub mod init;

/// Main DigitalOcean struct
pub struct DigitalOcean(Arc<SharedDigitalOcean>);

impl Clone for DigitalOcean {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for DigitalOcean {
    type Target = SharedDigitalOcean;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct SharedDigitalOcean {
    token: String,
    client: Client,
}
