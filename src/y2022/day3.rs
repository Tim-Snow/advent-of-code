use std::{collections::HashSet, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

pub async fn run() {
    let data = get_day_data(3, 2022).await;
    let test_data = get_day_test_data(3, 2022);

    fn char_priority(c: char) -> u8 {
        if c.is_uppercase() {
            return (c as u8) - 38u8;
        }
        (c as u8) - 96u8
    }

    fn part_one(d: &str) -> String {
        d.lines()
            .map(|rucksack| -> u8 {
                let len = rucksack.len();
                let first_compartment = &rucksack[0..len / 2];
                let second_compartment = &rucksack[len / 2..];

                first_compartment
                    .chars()
                    .filter(|&c| second_compartment.contains(c))
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .map(char_priority)
                    .fold(u8::MIN, |acc, cur| acc + cur)
            })
            .fold(u16::MIN, |acc, cur| acc + (cur as u16))
            .to_string()
    }

    fn part_two(d: &str) -> String {
        d.lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|chunk| -> u8 {
                let (first, second, third) = (chunk[0], chunk[1], chunk[2]);
                first
                    .chars()
                    .filter(|&c| second.contains(c) && third.contains(c))
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .fold(u8::MIN, |acc, cur| acc + char_priority(cur))
            })
            .fold(u16::MIN, |acc, cur| acc + (cur as u16))
            .to_string()
    }

    check_results((part_one(&test_data), "157"), (part_two(&test_data), "70"));

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(3, 2022, &part_one, &part_two, started);

    check_results((part_one, "7903"), (part_two, "2548"));
}
