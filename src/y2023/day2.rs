use std::{str::FromStr, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

struct Bag {
    red_cubes: u8,
    green_cubes: u8,
    blue_cubes: u8,
}

impl Bag {
    fn under_limit(&self) -> bool {
        self.red_cubes.le(&12) && self.green_cubes.le(&13) && self.blue_cubes.le(&14)
    }
}

struct Game {
    id: u8,
    bag_maxes: Bag,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let mut game = split.next().unwrap().split_whitespace();
        let results: Vec<&str> = split
            .next()
            .unwrap()
            .split(&[',', ';'])
            .map(str::trim)
            .collect();

        let id = game.nth(1).unwrap().parse::<u8>().unwrap();

        let mut blue_cubes: u8 = 0;
        let mut green_cubes: u8 = 0;
        let mut red_cubes: u8 = 0;

        results.iter().for_each(|result| {
            let mut split = result.split_whitespace();
            let amount = split.next().unwrap().parse::<u8>().unwrap();
            let colour = split.next().unwrap();

            if colour.eq("blue") && amount > blue_cubes {
                blue_cubes = amount;
            }
            if colour.eq("green") && amount > green_cubes {
                green_cubes = amount;
            }
            if colour.eq("red") && amount > red_cubes {
                red_cubes = amount;
            }
        });

        Ok(Game {
            id,
            bag_maxes: Bag {
                blue_cubes,
                green_cubes,
                red_cubes,
            },
        })
    }
}

pub async fn run() {
    let data = get_day_data(2, 2023).await;
    let test_data = get_day_test_data(2, 2023);

    fn part_one(d: &str) -> String {
        d.lines()
            .fold(u16::MIN, |acc, curr| {
                let game = curr.parse::<Game>().unwrap();

                if game.bag_maxes.under_limit() {
                    acc + u16::from(game.id)
                } else {
                    acc
                }
            })
            .to_string()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    check_results((part_one(&test_data), "8"), (part_two(&test_data), ""));

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(2, 2023, &part_one, &part_two, started);

    check_results((part_one, ""), (part_two, ""));
}
