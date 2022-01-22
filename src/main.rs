mod digitalocean;

use digitalocean::DigitalOcean;

fn main() {
    dotenv::dotenv().ok();

    let token = dotenv::var("DO_TOKEN").unwrap();
    // println!("{}", token);

    // let rate_limit = dotenv::var("DO_RATE_LIMIT").unwrap().parse::<u16>().unwrap();

    // for i in 1..=20 {
    //     let interval = calculate_polling_interval(rate_limit, i);
    //     println!("For {} instances polling can be every {} secs", i, interval);
    // } 

    // let rate_limit: u16 = 5000;
    // let calc = rate_limit / 60 / 4;
    // println!("{}", calc);

    let digitalocean = DigitalOcean::auth(token);
    let deployments = digitalocean.deployments("x123");
    println!("{:?}", deployments);
}

// Polling every n secs
fn _calculate_polling_interval(rate_limit: u16, apps_num: u16) -> u16 {
    60 / (rate_limit  / 60 / apps_num) + 5
}