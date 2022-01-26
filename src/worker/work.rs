//! Worker working.

use tokio::task;

use crate::worker::Worker;

impl Worker {
    // Initializes worker
    pub async fn work(&self) {
        if let Err(e) = work(self).await {
            log::warn!("{}", e);
        }
    }
}

async fn work(worker: &Worker) -> anyhow::Result<()> {
    let apps = worker.digitalocean.apps().get().await?;

    let mut handles = Vec::new();

    for app in apps {
        let w = worker.clone();

        let handle = task::spawn(async move {
            if let Err(e) = task(w, &app.id).await {
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

async fn task(worker: Worker, app_id: &str) -> anyhow::Result<()> {
    // Check if the table exists
    if !worker.database.table(app_id).exists() {
        log::info!("A new App has been detected");

        // Create a table
        worker.database.table(app_id).create().await?;

        // Get deployments
        let deployments = worker.digitalocean.deployments().get(app_id).await?;

        // Write data to the table
        let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
        worker.database.table(app_id).write(data).await?;

        // Telegram!!!!!

        return Ok(());
    }

    // Create a table
    worker.database.table(app_id).create().await?;

    // Get deployments
    let deployments = worker.digitalocean.deployments().get(app_id).await?;

    // Write data to the table
    let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
    worker.database.table(app_id).write(data).await?;

    Ok(())
}
