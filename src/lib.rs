//! Lib module

use database::Database;
use digitalocean::DigitalOcean;
use logging::Logging;
use telegram::Telegram;
use worker::Worker;

pub mod database;
pub mod digitalocean;
pub mod logging;
pub mod telegram;
pub mod worker;

pub async fn run() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Initialize logging
    Logging::from_env().init()?;

    // // Create DigitalOcean instance
    // let digitalocean = DigitalOcean::from_env().init().await?;

    // // Create DataBase instance
    // let database = Database::from_env().init().await?;

    // // Create Telegram instance
    // // let telegram = Telegram::from_env().init().await.unwrap();

    // // Create Worker instance
    // let worker = Worker::from_env().init(digitalocean, database).await;

    // // Run monitoring
    // worker.work().await;

    let telegram = Telegram::from_env().init().await?;
    telegram.message("123").send().await?;

    Ok(())
}
