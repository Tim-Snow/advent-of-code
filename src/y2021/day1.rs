use std::time::Instant;

use crate::util::{get_day_data, get_day_test_data, log_result};

pub async fn run() {
    let data = get_day_data(1, 2021).await;
    let test_data = get_day_test_data(1, 2021);

    fn part_one(d: &str) -> String {
        d.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u32>().unwrap())
            .fold((u32::MIN, u32::MAX), |(accum, prev), num| {
                match num.gt(&prev) {
                    true => (accum + 1, num),
                    false => (accum, num),
                }
            })
            .0
            .to_string()
    }

    fn part_two(d: &str) -> String {
        d.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u32>().unwrap())
            .fold(
                (u32::MIN, (None, None, None)),
                |(accum, previous), current| match previous {
                    (None, None, None) => (0, (None, None, Some(current))),
                    (None, None, Some(k)) => (0, (None, Some(k), Some(current))),
                    (None, Some(j), Some(k)) => (0, (Some(j), Some(k), Some(current))),
                    (Some(i), Some(j), Some(k)) => {
                        let prev_sum = i + j + k;
                        let current_sum = j + k + current;
                        let next = (Some(j), Some(k), Some(current));

                        match current_sum.gt(&prev_sum) {
                            true => (accum + 1, next),
                            false => (accum, next),
                        }
                    }
                    _ => unreachable!(),
                },
            )
            .0
            .to_string()
    }

    assert_eq!(part_one(&test_data), "7");
    assert_eq!(part_two(&test_data), "5");

    log_result(1, 2021, &part_one(&data), &part_two(&data), Instant::now())
}
