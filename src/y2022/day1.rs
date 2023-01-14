use std::time::Instant;

use crate::util::{get_day_data, get_day_test_data, log_result, LINE_ENDING};

pub async fn run() {
    let data = get_day_data(1, 2022).await;
    let test_data = get_day_test_data(1, 2022);

    fn sum_calories(d: &str) -> Vec<u32> {
        let mut values: Vec<u32> = d
            .split(&format!("{LINE_ENDING}{LINE_ENDING}"))
            .map(|set| {
                set.lines()
                    .filter(|line| !line.is_empty())
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect();

        values.sort();

        values
    }

    fn part_one(d: &str) -> String {
        sum_calories(d).iter().rev().next().unwrap().to_string()
    }

    fn part_two(d: &str) -> String {
        sum_calories(d)
            .iter()
            .rev()
            .take(3)
            .sum::<u32>()
            .to_string()
    }

    assert_eq!(part_one(&test_data), "24000");
    assert_eq!(part_two(&test_data), "45000");

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    assert_eq!(part_one, "70374");
    assert_eq!(part_two, "204610");

    log_result(1, 2022, &part_one, &part_two, started);
}
