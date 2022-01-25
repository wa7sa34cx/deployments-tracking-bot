//! Main module

use telegram_devops_bot::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
