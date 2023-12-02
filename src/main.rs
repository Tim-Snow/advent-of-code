mod util;
mod y2021;
mod y2022;
mod y2023;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let year = dotenv::var("YEAR");

    match year {
        Ok(year) => match year.parse::<u16>() {
            Ok(year) => match year {
                2015..=2020 => unimplemented!("Year {year} not implemented"),
                2021 => y2021::run().await,
                2022 => y2022::run().await,
                2023 => y2023::run().await,
                _ => panic!("Invalid year: {year}"),
            },
            Err(_) => println!("Could not parse year: '{year}' to int"),
        },
        Err(_) => println!("Year not provided"),
    }
}
