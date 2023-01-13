use std::{ops::AddAssign, time::Instant};

use crate::util::{get_day_data, get_day_test_data, log_result};

pub async fn run() {
    let data = get_day_data(1, 2022).await;
    let test_data = get_day_test_data(1, 2022);

    fn part_one(d: &str) -> String {
        let mut highest = u32::MIN;
        let mut current = u32::MIN;

        d.lines().for_each(|line| match line.parse::<u32>() {
            Err(_) => {
                if current.gt(&highest) {
                    highest = current;
                }
                current = 0;
            }
            Ok(num) => current.add_assign(num),
        });

        highest.to_string()
    }

    fn part_two(d: &str) -> String {
        String::new()
    }

    assert_eq!(part_one(&test_data), "24000");
    assert_eq!(part_two(&test_data), "45000");

    log_result(1, 2022, &part_one(&data), &part_two(&data), Instant::now())
}
