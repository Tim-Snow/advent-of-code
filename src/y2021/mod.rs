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
        Ok(d) => match d.parse::<u8>() {
            Ok(d) => match d {
                1 => day1::run().await,
                2..=25 => panic!("Day {d} not implemented"),
                _ => panic!("Invalid day: {d}"),
            },
            Err(_) => {
                println!("Could not parse day: '{d}' to int, running all");
                join![day1::run()];
            }
        },
    }
}
