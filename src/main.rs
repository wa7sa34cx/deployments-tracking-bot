//! Main module

use deployments_tracking_bot::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
