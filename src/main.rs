mod util;
mod y2021;
mod y2022;

use futures::join;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let year = dotenv::var("YEAR");

    match year {
        Err(_) => {
            println!("Year not provided, running all");
            join!(y2021::run(), y2022::run());
        }
        Ok(year) => match year.parse::<u16>() {
            Ok(year) => match year {
                2015..=2020 => panic!("Year {year} not implemented"),
                2021 => y2021::run().await,
                2022 => y2022::run().await,
                _ => panic!("Invalid year: {year}"),
            },
            Err(_) => {
                println!("Could not parse year: '{year}' to int, running all");
                join!(y2021::run(), y2022::run());
            }
        },
    }
}
