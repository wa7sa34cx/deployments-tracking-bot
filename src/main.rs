mod digitalocean;
mod logging;

use digitalocean::DigitalOcean;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Initialize logging
    logging::init();

    // Get token from environment
    let token = dotenv::var("DO_TOKEN").unwrap();

    // Create keep-alive HTTP connection pool
    let client = reqwest::Client::new();

    // Create DigitalOcean instance
    let digitalocean = DigitalOcean::auth(token, client);
    // println!("{:?}", digitalocean);
    // let deployments = digitalocean.deployments("x123");
    // println!("{:?}", deployments);

    // !!!! Panic!!!
    let apps = digitalocean.get_apps().await.unwrap();
    println!("{:?}", apps);
}

// Polling every n secs
// fn _calculate_polling_interval(rate_limit: u16, apps_num: u16) -> u16 {
//     60 / (rate_limit  / 60 / apps_num) + 5
// }
