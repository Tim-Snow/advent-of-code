mod util;
mod y2021;
mod y2022;

use futures::join;

#[tokio::main]

async fn main() {
    dotenv::dotenv().ok();

    let year = dotenv::var("YEAR");

    match year {
        Ok(y) => match y.parse::<u16>().unwrap() {
            2021 => y2021::run().await,
            2022 => y2022::run().await,
            _ => panic!("Invalid year: {}", y),
        },
        Err(_) => {
            join!(y2021::run(), y2022::run());
        }
    }
}
