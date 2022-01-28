//! Heavy Hard Work Module.
//!
//! A lot of people failed at what you accomplished,
//! simply because they were busy finding problems
//! while you were busy finding solutions. Well done.

use tokio::{
    task,
    time::{self, Duration},
};

use crate::digitalocean::models::{
    app::App,
    deployment::{Deployment, MsgType},
};
use crate::worker::Worker;

impl Worker {
    /// Gets to work!
    pub async fn work(&self) {
        log::info!("deployments monitoring has been successfully run");

        let mut interval = time::interval(Duration::from_secs(self.config.interval));

        // Main worker loop
        loop {
            interval.tick().await;

            log::debug!("checking for new deployments...");

            if let Err(e) = work(self).await {
                log::warn!("{}", e);
            }
        }
    }
}

async fn work(worker: &Worker) -> anyhow::Result<()> {
    let apps = worker.digitalocean.apps().get().await?;

    let mut handles = Vec::new();

    for app in apps {
        let w = worker.clone();

        let handle = task::spawn(async move {
            if let Err(e) = task(w, app).await {
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

async fn task(worker: Worker, app: App) -> anyhow::Result<()> {
    // Check if the table doesn't exists
    if !worker.database.table(&app.id).exists() {
        log::info!("a new App ({}) has been detected", &app.id);

        // Create a table
        worker.database.table(&app.id).create().await?;
    }

    // Get deployments
    let deployments = worker.digitalocean.deployments().get(&app).await?;

    // Get deployments from table
    let deployments_current = worker.database.table(&app.id).read().await?;

    // Search for new deployments and send notification
    let mut was_found = false;
    for deployment in deployments.iter() {
        if !deployments_current.contains(&deployment.id) {
            log::info!("a new deployment ({}) has been detected", &deployment.id);
            send_messages(&worker, &deployment).await?;
            was_found = true;
        }
    }

    // Write new data to the table if new deployments was found
    if was_found {
        let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
        worker.database.table(&app.id).write(data).await?;
    }

    Ok(())
}

// Sends messages
async fn send_messages(worker: &Worker, deployment: &Deployment) -> anyhow::Result<()> {
    // 1. Send message to Telegram
    let message = deployment.message(MsgType::Telegram).await?;
    worker.telegram.message(&message).send().await?;
    
    log::info!(
        "message about deployment ({}) has been successfully send to Telegram bot",
        &deployment.id
    );

    // 2. Send message to... Discord?
    // let message = deployment.message(MsgType::Discord).await?;
    // worker.discord.message(&message).send().await?;

    // And we can do it in parallel mode 😉

    Ok(())
}
