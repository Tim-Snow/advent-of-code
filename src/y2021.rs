use futures::join;

pub mod day1;

pub async fn run() {
    println!("Year 2021");

    let day = dotenv::var("DAY");

    match day {
        Ok(d) => match d.parse::<u8>() {
            Ok(d) => match d {
                1 => day1::run().await,
                2..=25 => panic!("Day {} - Not implemented", d),
                _ => panic!("Invalid day: {}", d),
            },
            Err(_) => {
                join![day1::run()];
            }
        },
        Err(_) => {
            join![day1::run()];
        }
    }
}
