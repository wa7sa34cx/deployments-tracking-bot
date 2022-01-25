//! Worker initialization.

use tokio::task;

use crate::database::Database;
use crate::digitalocean::DigitalOcean;
use crate::worker::{Worker, WorkerConfig};

impl WorkerConfig {
    // Initializes worker
    pub async fn init(self, digitalocean: DigitalOcean, database: Database) -> Worker {
        if let Err(e) = init(digitalocean.clone(), database.clone()).await {
            log::warn!("{}", e);
        }

        Worker {
            digitalocean,
            database,
            config: self,
        }
    }
}

async fn init(digitalocean: DigitalOcean, database: Database) -> anyhow::Result<()> {
    let apps = digitalocean.apps().get().await?;
            
    let mut handles = Vec::new();

    for app in apps {
        let dn = digitalocean.clone();
        let db = database.clone();

        let handle = task::spawn(async move {
            if let Err(e) = task(dn, db, &app.id).await {
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

async fn task(digitalocean: DigitalOcean, database: Database, app_id: &str) -> anyhow::Result<()> {
    // Create a table
    database.table(app_id).create().await?;

    // Get deployments
    let deployments = digitalocean.deployments().get(app_id).await?;
    
    // Write to the talbe 
    let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
    database.table(app_id).write(data).await?;

    Ok(())
}