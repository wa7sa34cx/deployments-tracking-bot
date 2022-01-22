use crate::digitalocean::DigitalOcean;

#[derive(Debug)]
pub struct Deployments {
    pub response: String,
}

impl DigitalOcean {
    pub fn deployments(&self, app_id: impl Into<String>) -> Deployments {
        Deployments {
            response: format!("Response: {}", app_id.into()),
        }
    }
}

