//! Lib module

use database::Database;
use digitalocean::DigitalOcean;
use logging::Logging;
// use telegram::Telegram;
use tokio::task;

pub mod database;
pub mod digitalocean;
pub mod logging;

pub async fn run() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Initialize logging
    Logging::from_env().init().unwrap();

    // Create DigitalOcean instance
    let digitalocean = DigitalOcean::from_env().init().await.unwrap();

    // Create DataBase instance
    let database = Database::from_env().init().await.unwrap();

    // Create Telegram instance
    // let telegram = Telegram::from_env().init().await.unwrap();

    // Apps
    let apps = match digitalocean.apps().get().await {
        Ok(apps) => apps,
        Err(e) => {
            log::error!("{}", e);
            return Ok(());
        }
    };

    println!("{:#?}", apps);

    //-------- TRY PARALLELISM ------
    let mut handles = Vec::new();

    for app in apps {

        let docean = digitalocean.clone();
        let dbase = database.clone();

        let handle = task::spawn(async move {
            dbase.table(&app.id).create().await.unwrap();
            dbase.table(&app.id).exists();

            let deployments = docean.deployments().get(&app.id).await.unwrap();

            let data: Vec<&str> = deployments.iter().map(|d| d.id.as_str()).collect();
            dbase.table(&app.id).write(data).await.unwrap();
        });

        handles.push(handle);
    }

    // Await for all tasks
    for handle in handles {
        handle.await.unwrap();
    }



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

// Polling every n secs
fn _calculate_gap(rate_limit: u16, apps_num: u16) -> u16 {
    60 / (rate_limit / 60 / (apps_num + 1)) + 5
}
