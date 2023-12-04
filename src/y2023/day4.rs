use std::{str::FromStr, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

struct Card {
    _id: u8,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let id = split
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let mut split = split.next().unwrap().split('|');
        let winning_numbers = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u8>().unwrap())
            .collect();
        let numbers = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u8>().unwrap())
            .collect();

        Ok(Card {
            _id: id,
            winning_numbers,
            numbers,
        })
    }
}

impl Card {
    fn number_of_matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
    }

    fn score(&self) -> usize {
        let matches = self.number_of_matches();

        match matches {
            0 => 0,
            _ => 1 << (matches - 1),
        }
    }
}

pub async fn run() {
    let data = get_day_data(4, 2023).await;
    let test_data = get_day_test_data(4, 2023);

    fn part_one(d: &str) -> String {
        let cards = d.lines().map(|line| line.parse::<Card>().unwrap());

        cards
            .fold(u16::MIN, |acc, curr| {
                acc + u16::try_from(curr.score()).unwrap()
            })
            .to_string()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    check_results((part_one(&test_data), "13"), (part_two(&test_data), ""));

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(4, 2023, &part_one, &part_two, started);

    check_results((part_one, "21919"), (part_two, ""));
}
