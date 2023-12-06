use std::{str::FromStr, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

struct Race {
    time: u16,
    distance: u16,
}

struct Data {
    races: Vec<Race>,
}

impl FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let times: Vec<u16> = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u16>().unwrap())
            .collect();

        let distances: Vec<u16> = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u16>().unwrap())
            .collect();

        let mut races: Vec<Race> = Vec::new();

        for (i, time) in times.iter().enumerate() {
            races.push(Race {
                time: *time,
                distance: *distances.get(i).unwrap(),
            })
        }

        Ok(Data { races })
    }
}

impl Data {
    fn calculate_result(&self) -> u32 {
        self.races
            .iter()
            .map(Self::number_of_ways_to_win)
            .map(u32::from)
            .product()
    }

    fn number_of_ways_to_win(race: &Race) -> u16 {
        let mut ways_to_win = u16::MIN;

        for i in 0..race.distance {
            let speed: u16 = i;
            if i < race.time {
                let time_left: u16 = race.time - i;
                let distance_travelled = speed * time_left;

                if distance_travelled > race.distance {
                    ways_to_win += 1;
                }
            }
        }

        ways_to_win
    }
}

pub async fn run() {
    let data = get_day_data(6, 2023).await;
    let test_data = get_day_test_data(6, 2023);

    fn part_one(d: &str) -> String {
        let races = d.parse::<Data>().unwrap();
        races.calculate_result().to_string()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    check_results((part_one(&test_data), "288"), (part_two(&test_data), ""));

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(6, 2023, &part_one, &part_two, started);

    check_results((part_one, "303600"), (part_two, ""));
}
