//! Worker working.

use tokio::task;

use crate::database::Database;
use crate::digitalocean::DigitalOcean;
use crate::worker::{Worker, WorkerConfig};

impl Worker {
    // Initializes worker
    pub async fn work(&self) -> anyhow::Result<Worker> {
        let apps = digitalocean.apps().get().await?;

        let mut handles = Vec::new();

        for app in apps {
            let dn = digitalocean.clone();
            let db = database.clone();

            let handle = task::spawn(async move {
                db.table(&app.id).create().await.unwrap();

                let deployments = dn.deployments().get(&app.id).await.unwrap();

                let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
                db.table(&app.id).write(data).await.unwrap();
            });

            handles.push(handle);
        }

        // Await for all tasks
        for handle in handles {
            handle.await?;
        }

        Ok(Worker {
            digitalocean,
            database,
            config: self,
        })
    }
}

// Polling every n secs
fn _calculate_gap(rate_limit: u16, apps_num: u16) -> u16 {
    60 / (rate_limit / 60 / (apps_num + 1)) + 5
}
