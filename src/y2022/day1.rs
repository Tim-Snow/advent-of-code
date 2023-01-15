use std::time::Instant;

use crate::util::{check_results, get_day_data, get_day_test_data, log_results, LINE_ENDING};

pub async fn run() {
    let data = parse(&get_day_data(1, 2022).await);
    let test_data = parse(&get_day_test_data(1, 2022));

    fn parse(d: &str) -> Vec<u32> {
        sum_calories(d).into_iter().rev().collect()
    }

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

    fn part_one(d: &[u32]) -> String {
        d.first().unwrap().to_string()
    }

    fn part_two(d: &[u32]) -> String {
        d.iter().take(3).sum::<u32>().to_string()
    }

    check_results(
        (part_one(&test_data), "24000"),
        (part_two(&test_data), "45000"),
    );

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(1, 2022, &part_one, &part_two, started);

    check_results((part_one, "70374"), (part_two, "204610"));
}
