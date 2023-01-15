use std::{str::FromStr, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_result};

struct FromTo {
    from: u8,
    to: u8,
}

struct Assignments {
    left: FromTo,
    right: FromTo,
}

impl FromStr for Assignments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');

        let mut left = split.next().unwrap().split('-');
        let left_from: u8 = left.next().unwrap().parse().unwrap();
        let left_to: u8 = left.next().unwrap().parse().unwrap();

        let mut right = split.next().unwrap().split('-');
        let right_from: u8 = right.next().unwrap().parse().unwrap();
        let right_to: u8 = right.next().unwrap().parse().unwrap();

        Ok(Assignments {
            left: FromTo {
                from: left_from,
                to: left_to,
            },
            right: FromTo {
                from: right_from,
                to: right_to,
            },
        })
    }
}

impl Assignments {
    fn fully_contains(&self) -> bool {
        self.left.from <= self.right.from && self.left.to >= self.right.to
            || self.left.from >= self.right.from && self.left.to <= self.right.to
    }

    fn overlaps(&self) -> bool {
        self.left.from <= self.right.from && self.left.to >= self.right.from
            || self.right.from <= self.left.from && self.right.to >= self.left.from
    }
}

pub async fn run() {
    let data = get_day_data(4, 2022).await;
    let test_data = get_day_test_data(4, 2022);

    fn part_one(d: &str) -> u16 {
        d.lines()
            .map(|line| -> u16 {
                let assignments: Assignments = line.parse().unwrap();

                match assignments.fully_contains() {
                    true => 1,
                    false => 0,
                }
            })
            .sum()
    }

    fn part_two(d: &str) -> u16 {
        d.lines()
            .map(|line| -> u16 {
                let assignments: Assignments = line.parse().unwrap();

                match assignments.overlaps() {
                    true => 1,
                    false => 0,
                }
            })
            .sum()
    }

    check_results(
        (part_one(&test_data).to_string(), "2"),
        (part_two(&test_data).to_string(), "4"),
    );

    let started = Instant::now();

    let part_one = part_one(&data).to_string();
    let part_two = part_two(&data).to_string();

    log_result(4, 2022, &part_one, &part_two, started);

    check_results((part_one, "573"), (part_two, "867"));
}
