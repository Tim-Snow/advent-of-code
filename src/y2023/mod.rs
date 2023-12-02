pub async fn run() {
    let day = dotenv::var("DAY");

    match day {
        Ok(day) => match day.parse::<u8>() {
            Ok(day) => match day {
                1..=25 => not_implemented(day),
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
