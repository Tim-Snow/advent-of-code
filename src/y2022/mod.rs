mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;

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
                6..=9 => not_implemented(day),
                10 => day10::run().await,
                11 => day11::run().await,
                12 => day12::run().await,
                13 => day13::run().await,
                14..=25 => not_implemented(day),
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
