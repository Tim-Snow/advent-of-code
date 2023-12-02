use crate::util::{check_results, get_day_data, get_day_test_data, log_results};
use std::fs::read_to_string;
use std::time::Instant;

pub async fn run() {
    let data = get_day_data(1, 2023).await;
    let test_data = get_day_test_data(1, 2023);
    let test_data2 = read_to_string("res/2023/1.2.test.txt").expect("Test data has been created");

    fn part_one(d: &str) -> String {
        d.lines()
            .fold(u16::MIN, |acc, curr| {
                let first_digit = curr.chars().find(|c| c.is_numeric()).unwrap();
                let last_digit = curr.chars().filter(|c| c.is_numeric()).next_back().unwrap();

                let concatenated = format!("{}{}", first_digit, last_digit);

                acc + concatenated.parse::<u16>().unwrap()
            })
            .to_string()
    }

    fn part_two(d: &str) -> String {
        let numbers_map = [
            ("one", 1),
            ("1", 1),
            ("two", 2),
            ("2", 2),
            ("three", 3),
            ("3", 3),
            ("four", 4),
            ("4", 4),
            ("five", 5),
            ("5", 5),
            ("six", 6),
            ("6", 6),
            ("seven", 7),
            ("7", 7),
            ("eight", 8),
            ("8", 8),
            ("nine", 9),
            ("9", 9),
        ];

        d.lines()
            .fold(u16::MIN, |acc, curr| {
                let mut first_digit: Option<u8> = None;
                let mut last_digit: Option<u8> = None;

                for (index, _char) in curr.chars().enumerate() {
                    let slice = &curr[index..curr.len()];

                    numbers_map.map(|(number_string, number_value)| {
                        if slice.starts_with(number_string) {
                            if first_digit.is_none() {
                                first_digit = Some(number_value)
                            }
                            last_digit = Some(number_value)
                        }
                    });
                }

                let concatenated = format!("{:?}{:?}", first_digit.unwrap(), last_digit.unwrap());
                dbg!(first_digit, last_digit, &concatenated);

                acc + concatenated.parse::<u16>().unwrap()
            })
            .to_string()
    }

    check_results(
        (part_one(&test_data), "142"),
        (part_two(&test_data2), "281"),
    );

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(1, 2023, &part_one, &part_two, started);

    check_results((part_one, "56042"), (part_two, "55358"));
}
