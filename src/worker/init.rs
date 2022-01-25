//! Worker initialization.

use tokio::task;

use crate::database::Database;
use crate::digitalocean::DigitalOcean;
use crate::worker::{Worker, WorkerConfig};

impl WorkerConfig {
    // Initializes worker
    pub async fn init(self, digitalocean: DigitalOcean, database: Database) -> anyhow::Result<Worker> {
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