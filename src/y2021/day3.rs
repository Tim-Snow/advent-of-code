use std::time::Instant;

use crate::util::{get_day_data, log_results};

pub async fn run() {
    let data = get_day_data(3, 2021).await;

    fn calc_counts(str: &str) -> Vec<i32> {
        let length = str.lines().next().unwrap().len();
        let mut counts = vec![0; length];

        str.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(i, c)| match c {
                '0' => counts[i] -= 1,
                '1' => counts[i] += 1,
                _ => unreachable!(),
            })
        });

        counts
    }

    fn calc_bit(count: i32, if_char: char, else_char: char) -> char {
        if count.gt(&0) {
            if_char
        } else {
            else_char
        }
    }

    fn part_one(d: &str) -> String {
        let counts = calc_counts(d);

        let gamma_binary = &counts
            .iter()
            .map(|&count| calc_bit(count, '1', '0'))
            .collect::<String>();

        let gamma_rate = usize::from_str_radix(gamma_binary, 2).unwrap();

        let epsilon_rate = usize::from_str_radix(
            &gamma_binary
                .chars()
                .map(|char| calc_bit(char.to_digit(2).unwrap() as i32, '0', '1'))
                .collect::<String>(),
            2,
        )
        .unwrap();

        (gamma_rate * epsilon_rate).to_string()
    }

    fn part_two(_d: &str) -> String {
        String::from("todo!()")
    }

    log_results(3, 2021, &part_one(&data), &part_two(&data), Instant::now());
}
