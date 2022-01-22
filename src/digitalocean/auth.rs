use crate::digitalocean::DigitalOcean;

impl DigitalOcean {
    pub fn auth(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}