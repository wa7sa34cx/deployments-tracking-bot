//! Error respose from DigitalOcean

use serde_derive::Deserialize;

/// Error response
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// A short identifier corresponding to the HTTP status code returned.
    pub id: String,
    /// A message providing additional information about the error, 
    /// including details to help resolve it when possible.
    pub message: String,
}
