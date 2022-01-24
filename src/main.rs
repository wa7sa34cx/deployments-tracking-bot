//! Main module

use database::Database;
// use digitalocean::DigitalOcean;

mod database;
mod digitalocean;
mod logging;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Initialize logging
    logging::init();

    // // Get token from environment
    // let token = dotenv::var("DO_TOKEN").unwrap();

    // // Create keep-alive HTTP connection pool
    // let client = reqwest::Client::new();

    // // Create DigitalOcean instance
    // let digitalocean = DigitalOcean::auth(token, client);

    // // Apps
    // let apps = match digitalocean.get_apps().await {
    //     Ok(apps) => apps,
    //     Err(e) => {
    //         log::error!("{}", e);
    //         return;
    //     }
    // };

    // println!("{:#?}", apps);

    // // Deployments
    // let deployments = digitalocean.get_deployments(&apps[3]).await.unwrap();

    // println!("{:#?}", deployments);

    // --------------
    // Test DB

    let _database = Database::init().await.unwrap();


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
}

// Polling every n secs
fn _calculate_gap(rate_limit: u16, apps_num: u16) -> u16 {
    60 / (rate_limit / 60 / (apps_num + 1)) + 5
}
