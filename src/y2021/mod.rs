use futures::join;

mod day1;
mod day2;
mod day3;

pub async fn run() {
    println!("Year 2021");

    let day = dotenv::var("DAY");

    async fn run_all() {
        join![day1::run(), day2::run(), day3::run()];
    }

    match day {
        Err(_) => {
            println!("Day not provided, running all");
            run_all().await;
        }
        Ok(day) => match day.parse::<u8>() {
            Ok(day) => match day {
                1 => day1::run().await,
                2 => day2::run().await,
                3 => day3::run().await,
                4..=25 => panic!("Day {day} not implemented"),
                _ => panic!("Invalid day: {day}"),
            },
            Err(_) => {
                println!("Could not parse day: '{day}' to int, running all");
                run_all().await;
            }
        },
    }
}
