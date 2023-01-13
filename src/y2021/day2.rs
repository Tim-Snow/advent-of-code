use std::{ops::Mul, time::Instant};

use crate::util::{get_day_data, log_result};

enum Instruction {
    Forward(u16),
    Down(u16),
    Up(u16),
}

impl Instruction {
    fn new(from: &str) -> Instruction {
        let mut split = from.split_whitespace();
        let instruction = split.next().unwrap();
        let amount: u16 = split.next().unwrap().parse().unwrap();

        match instruction {
            "forward" => Instruction::Forward(amount),
            "up" => Instruction::Up(amount),
            "down" => Instruction::Down(amount),
            _ => unreachable!(),
        }
    }
}

pub async fn run() {
    let data = get_day_data(2, 2021).await;

    fn part_one(d: &str) -> String {
        let mut horizontal_position: u16 = 0;
        let mut depth: u16 = 0;

        d.lines().for_each(|line| match Instruction::new(line) {
            Instruction::Down(amount) => depth += amount,
            Instruction::Forward(amount) => horizontal_position += amount,
            Instruction::Up(amount) => depth -= amount,
        });

        u32::from(horizontal_position)
            .mul(u32::from(depth))
            .to_string()
    }

    fn part_two(d: &str) -> String {
        let mut horizontal_position: u16 = 0;
        let mut depth: u32 = 0;
        let mut aim: u16 = 0;

        d.lines().for_each(|line| match Instruction::new(line) {
            Instruction::Down(amount) => aim += amount,
            Instruction::Up(amount) => aim -= amount,
            Instruction::Forward(amount) => {
                horizontal_position += amount;
                depth += u32::from(aim).mul(u32::from(amount));
            }
        });

        u32::from(horizontal_position).mul(depth).to_string()
    }

    log_result(2, 2021, &part_one(&data), &part_two(&data), Instant::now());
}
