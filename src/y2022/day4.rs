use std::{str::FromStr, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

struct Window {
    start: u8,
    end: u8,
}

impl FromStr for Window {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let start: u8 = split.next().unwrap().parse().unwrap();
        let end: u8 = split.next().unwrap().parse().unwrap();

        Ok(Window { start, end })
    }
}

struct Assignments {
    a: Window,
    b: Window,
}

impl FromStr for Assignments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');

        let a: Window = split.next().unwrap().parse().unwrap();
        let b: Window = split.next().unwrap().parse().unwrap();

        Ok(Assignments { a, b })
    }
}

impl Assignments {
    fn fully_contains(&self) -> bool {
        self.a.start <= self.b.start && self.a.end >= self.b.end
            || self.a.start >= self.b.start && self.a.end <= self.b.end
    }

    fn overlaps(&self) -> bool {
        self.a.start <= self.b.start && self.a.end >= self.b.start
            || self.b.start <= self.a.start && self.b.end >= self.a.start
    }
}

pub async fn run() {
    let data = get_day_data(4, 2022).await;
    let test_data = get_day_test_data(4, 2022);

    fn part_one(d: &str) -> String {
        d.lines()
            .map(|line| u16::from(line.parse::<Assignments>().unwrap().fully_contains()))
            .sum::<u16>()
            .to_string()
    }

    fn part_two(d: &str) -> String {
        d.lines()
            .map(|line| u16::from(line.parse::<Assignments>().unwrap().overlaps()))
            .sum::<u16>()
            .to_string()
    }

    check_results(
        (part_one(&test_data).to_string(), "2"),
        (part_two(&test_data).to_string(), "4"),
    );

    let started = Instant::now();

    let part_one = part_one(&data).to_string();
    let part_two = part_two(&data).to_string();

    log_results(4, 2022, &part_one, &part_two, started);

    check_results((part_one, "573"), (part_two, "867"));
}
