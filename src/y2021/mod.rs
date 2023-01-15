use futures::join;

mod day1;
mod day2;
mod day3;

pub async fn run() {
    let day = dotenv::var("DAY");

    match day {
        Err(_) => {
            println!("Day not provided");
            run_all().await;
        }
        Ok(day) => match day.parse::<u8>() {
            Ok(day) => match day {
                1 => day1::run().await,
                2 => day2::run().await,
                3 => day3::run().await,
                4..=25 => unimplemented!("Day {day} not implemented"),
                _ => panic!("Invalid day: {day}"),
            },
            Err(_) => {
                println!("Could not parse day: '{day}' to int");
                run_all().await;
            }
        },
    }

    async fn run_all() {
        println!("Running all");
        join![day1::run(), day2::run(), day3::run()];
    }
}
