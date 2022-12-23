use futures::join;

mod day1;
mod day2;
mod day3;
mod day5;

pub async fn run() {
    println!("Year 2022");

    let day = dotenv::var("DAY");

    match day {
        Err(_) => {
            println!("Day not provided, running all");
            join![day1::run(), day2::run(), day3::run(), day5::run()];
        }
        Ok(d) => match d.parse::<u8>() {
            Ok(d) => match d {
                1 => day1::run().await,
                2 => day2::run().await,
                3 => day3::run().await,
                4 => panic!("Day {d} not implemented"),
                5 => day5::run().await,
                6..=25 => panic!("Day {d} not implemented"),
                _ => panic!("Invalid day: {d}"),
            },
            Err(_) => {
                println!("Could not parse day: '{d}' to int, running all");
                join![day1::run(), day2::run(), day3::run(), day5::run()];
            }
        },
    }
}
