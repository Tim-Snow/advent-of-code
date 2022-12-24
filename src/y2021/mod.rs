use futures::join;

mod day1;

pub async fn run() {
    println!("Year 2021");

    let day = dotenv::var("DAY");

    match day {
        Err(_) => {
            println!("Day not provided, running all");
            join![day1::run()];
        }
        Ok(day) => match day.parse::<u8>() {
            Ok(day) => match day {
                1 => day1::run().await,
                2..=25 => panic!("Day {day} not implemented"),
                _ => panic!("Invalid day: {day}"),
            },
            Err(_) => {
                println!("Could not parse day: '{day}' to int, running all");
                join![day1::run()];
            }
        },
    }
}
