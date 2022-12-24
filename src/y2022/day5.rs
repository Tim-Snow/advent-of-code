use std::str::{FromStr, Lines};

use crate::util::get_day_data;

#[derive(Debug)]
struct Instruction {
    move_amount: u8,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut split = s.split_whitespace();

        let move_amount: u8 = split.nth(1).unwrap().parse().unwrap();
        let from: usize = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to: usize = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        Ok(Instruction {
            move_amount,
            from,
            to,
        })
    }
}

pub async fn run() {
    let data = get_day_data(5, 2022).await;
    // let test_data = read_to_string("res/5.txt.test").unwrap();

    fn parse(d: &str) -> (Vec<Vec<char>>, Lines) {
        let width = d
            .split("\n\n")
            .next()
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .len();
        let stacks_height = d.split("\n\n").next().unwrap().lines().count() - 1;
        let instructions = d.split("\n\n").last().unwrap().lines();

        let mut stacks: Vec<Vec<char>> = vec![];

        for x in (1..width).step_by(4) {
            stacks.push(vec![]);
            for y in 0..stacks_height {
                let c = d.lines().nth(y).unwrap().chars().nth(x);
                if let Some(c) = c {
                    if c != ' ' {
                        stacks.last_mut().unwrap().insert(0, c);
                    }
                }
            }
        }

        (stacks, instructions)
    }

    fn part_one(d: &str) -> String {
        let (mut stacks, instructions) = parse(d);

        for instruction in instructions {
            let parsed: Instruction = instruction.parse().unwrap();
            for _ in 0..parsed.move_amount {
                let moved = stacks.get_mut(parsed.from).unwrap().pop().unwrap();
                stacks.get_mut(parsed.to).unwrap().push(moved);
            }
        }

        let mut result = String::default();

        stacks.into_iter().for_each(|stack| {
            result.push(*stack.last().unwrap());
        });

        result
    }

    fn part_two(d: &str) -> String {
        let (mut stacks, instructions) = parse(d);

        for instruction in instructions {
            let parsed: Instruction = instruction.parse().unwrap();
            let mut moving: Vec<char> = vec![];

            for _ in 0..parsed.move_amount {
                let moved = stacks.get_mut(parsed.from).unwrap().pop().unwrap();
                moving.insert(0, moved);
            }

            stacks.get_mut(parsed.to).unwrap().append(&mut moving);
        }

        let mut result = String::default();

        stacks.into_iter().for_each(|stack| {
            result.push(*stack.last().unwrap());
        });

        result
    }

    // assert_eq!(part_one(&test_data), "CMZ");
    // assert_eq!(part_two(&test_data), "MCD");

    println!("1: {}\n2: {}", part_one(&data), part_two(&data));
}
