//! Lib module

use database::Database;
use digitalocean::DigitalOcean;
use logging::Logging;
use worker::Worker;
// use telegram::Telegram;

pub mod database;
pub mod digitalocean;
pub mod logging;
pub mod worker;

pub async fn run() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Initialize logging
    Logging::from_env().init()?;

    // Create DigitalOcean instance
    let digitalocean = DigitalOcean::from_env().init().await?;

    // Create DataBase instance
    let database = Database::from_env().init().await?;

    // Create Telegram instance
    // let telegram = Telegram::from_env().init().await.unwrap();

    // Create Worker instance
    let _worker = Worker::from_env().init(digitalocean, database).await;

    log::info!("deployments monitoring has been successfully run");

    // worker.work().await.unwrap();

    // 0. При запуске программы:
    // 0.1 Создать все базы данных
    // 0.2 Записать деплои в БД

    // LOOP

    // 1. Проверить существует ли база данных.

    // НЕТ
    // 1.1. Если нет, значит приложение новое, содать БД
    // 1.2. Послать сообщения о деплоях в Телеграм
    // 1.3. Создать БД
    // 1.4. Записать деплои в БД

    // ДА
    // 2.1. Если существует, то приложение старое
    // 2.2. Загрузить ID старых деплоев из БД
    // 2.3. Сверить все ID от API с ID из Базы
    // 2.4. Есть новые деплои?
    // 2.4.1 Нет => ничего не делать
    // 2.4.2 Да =>
    // 2.4.2.1 Послать сообщения о деплоях в Телеграм
    // 2.4.2.1 Записать деплои в БД

    // 2. GAP

    Ok(())
}
