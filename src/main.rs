mod util;
mod y2021;
mod y2022;

use futures::join;

#[tokio::main]

async fn main() {
    join!(y2021::year_run(), y2022::year_run());
}
