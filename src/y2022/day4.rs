use std::time::Instant;

use crate::util::{check_results, get_day_data, get_day_test_data, log_result};

pub async fn run() {
    let data = get_day_data(4, 2022).await;
    let test_data = get_day_test_data(4, 2022);

    fn part_one(d: &str) -> u16 {
        d.lines()
            .map(|line| -> u8 {
                let mut split = line.split(',');

                let mut one = split.next().unwrap().split('-');
                let one_from: u8 = one.next().unwrap().parse().unwrap();
                let one_to: u8 = one.next().unwrap().parse().unwrap();

                let mut two = split.next().unwrap().split('-');
                let two_from: u8 = two.next().unwrap().parse().unwrap();
                let two_to: u8 = two.next().unwrap().parse().unwrap();

                match one_from <= two_from && one_to >= two_to
                    || one_from >= two_from && one_to <= two_to
                {
                    true => 1,
                    false => 0,
                }
            })
            .fold(0, |acc, cur| acc + (cur as u16))
    }

    fn part_two(d: &str) -> u16 {
        fn range(of: &str) -> Vec<u8> {
            let mut split = of.split('-');
            let from: u8 = split.next().unwrap().parse().unwrap();
            let to: u8 = split.next().unwrap().parse().unwrap();

            let mut ret = Vec::new();

            for val in from..=to {
                ret.push(val);
            }

            ret
        }

        d.lines()
            .map(|line| -> u8 {
                let mut split = line.split(',');
                let one = range(split.next().unwrap());
                let two = range(split.next().unwrap());

                match one.iter().any(|value| two.contains(value)) {
                    true => 1,
                    false => 0,
                }
            })
            .fold(u16::MIN, |acc, cur| acc + (cur as u16))
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
