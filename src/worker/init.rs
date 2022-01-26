//! Worker initialization.

use tokio::task;

use crate::database::Database;
use crate::digitalocean::{models::app::App, DigitalOcean};
use crate::worker::{Worker, WorkerConfig};

impl WorkerConfig {
    // Initializes worker
    pub async fn init(self, digitalocean: DigitalOcean, database: Database) -> Worker {
        let worker = Worker {
            digitalocean,
            database,
            config: self,
        };

        if let Err(e) = init(worker.clone()).await {
            log::warn!("{}", e);
        }

        worker
    }
}

async fn init(worker: Worker) -> anyhow::Result<()> {
    let apps = worker.digitalocean.apps().get().await?;

    let mut handles = Vec::new();

    for app in apps {
        let w = worker.clone();

        let handle = task::spawn(async move {
            if let Err(e) = task(w, &app).await {
                log::warn!("{}", e);
            }
        });

        handles.push(handle);
    }

    // Await for all tasks
    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn task(worker: Worker, app: &App) -> anyhow::Result<()> {
    // Create a table
    worker.database.table(&app.id).create().await?;

    // Get deployments
    let deployments = worker.digitalocean.deployments().get(app).await?;

    // Write data to the table
    let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
    worker.database.table(&app.id).write(data).await?;

    Ok(())
}
