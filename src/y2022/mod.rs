mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
mod day9;

pub async fn run() {
    let day = dotenv::var("DAY");

    match day {
        Ok(day) => match day.parse::<u8>() {
            Ok(day) => match day {
                1 => day1::run().await,
                2 => day2::run().await,
                3 => day3::run().await,
                4 => day4::run().await,
                5 => day5::run().await,
                6 => not_implemented(day),
                7 => day7::run().await,
                8 => not_implemented(day),
                9 => day9::run().await,
                10 => day10::run().await,
                11 => day11::run().await,
                12 => day12::run().await,
                13 => day13::run().await,
                14 => not_implemented(day),
                15..=25 => not_implemented(day),
                _ => panic!("Invalid day: {day}"),
            },
            Err(_) => println!("Could not parse day: '{day}' to int"),
        },
        Err(_) => println!("Day not provided"),
    }

    fn not_implemented(day: u8) {
        unimplemented!("Day {day} not implemented");
    }
}
