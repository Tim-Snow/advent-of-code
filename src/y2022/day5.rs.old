use std::{ops::Sub, time::Instant};

use crate::util::{get_day_data, log_result};

pub async fn run() {
    let data = get_day_data(5, 2022).await;

    fn part_one(d: &str) -> String {
        let mut split = d.split("\r\n\r\n");
        let initial = split.next().unwrap();
        // let instructions = split.next().unwrap();

        let total_stacks: usize = initial
            .lines()
            .last()
            .unwrap()
            .trim()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();

        let mut stacks: Vec<Vec<char>> = vec![];

        for _ in 0..total_stacks {
            stacks.push(vec![]);
        }

        dbg!("{} {}", &stacks, total_stacks - 1);

        let mut cur_stack = 0;
        let mut prev_x = 1;
        for x in (1..initial.chars().count()).step_by(4) {
            for y in 0..total_stacks.sub(1) {
                let current_stack = stacks
                    .get_mut(cur_stack)
                    .expect(&dbg!(format!("{} {} {} {}", cur_stack, x, y, prev_x)));

                if x != prev_x {
                    prev_x = x;
                    cur_stack += 1;
                }

                let line = initial.lines().nth(y).unwrap();
                let c = line.chars().nth(x);

                if c.is_some() && c.unwrap().is_alphabetic() {
                    dbg!("{}", c.unwrap());
                    current_stack.push(match c {
                        Some(val) => val,
                        None => panic!("called `Option::unwrap()` on a `None` value"),
                    });
                }
            }
        }

        dbg!("{}", stacks);

        initial
            .lines()
            .for_each(|line| line.chars().for_each(|_char| {}));

        String::default()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    log_result(5, 2022, &part_one(&data), &part_two(&data), Instant::now());
}
