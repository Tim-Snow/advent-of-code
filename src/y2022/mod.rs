use futures::join;

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
    println!("Year 2022");

    let day = dotenv::var("DAY");

    async fn run_all() {
        println!("Running all");
        join![
            day1::run(),
            day2::run(),
            day3::run(),
            day4::run(),
            day5::run(),
            day10::run(),
            day11::run(),
            day12::run(),
            day13::run()
        ];
    }

    fn not_implemented(day: u8) {
        unimplemented!("Day {day} not implemented");
    }

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
            Err(_) => {
                println!("Could not parse day: '{day}' to int");
                run_all().await;
            }
        },
    }
}
