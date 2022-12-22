use std::time::Instant;

use crate::util::{get_day_data, log_result};

pub async fn run() {
    let data = get_day_data(1, 2022).await;

    println!("{}", data);

    log_result(1, 2022, "pt1", "pt2", Instant::now())
}
