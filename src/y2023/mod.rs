mod day1;
mod day2;
mod day4;

pub async fn run() {
    let day = dotenv::var("DAY");

    match day {
        Ok(day) => match day.parse::<u8>() {
            Ok(day) => match day {
                1 => day1::run().await,
                2 => day2::run().await,
                3 => not_implemented(day),
                4 => day4::run().await,
                5..=25 => not_implemented(day),
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
