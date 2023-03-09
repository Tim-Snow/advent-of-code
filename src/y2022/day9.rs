use std::{collections::HashSet, ops::AddAssign, str::FromStr, time::Instant};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    x: i16,
    y: i16,
}

impl Coord {
    fn new() -> Self {
        Coord { x: 0, y: 0 }
    }

    fn approach(&mut self, other: &Coord) {
        match (self.x.abs_diff(other.x), self.y.abs_diff(other.y)) {
            (2, 0) => self.x.add_assign((other.x - self.x) / 2),
            (2, 1) => {
                self.x.add_assign((other.x - self.x) / 2);
                self.y.add_assign(other.y - self.y);
            }
            (0, 2) => self.y.add_assign((other.y - self.y) / 2),
            (1, 2) => {
                self.x.add_assign(other.x - self.x);
                self.y.add_assign((other.y - self.y) / 2);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct HeadTail {
    head: Coord,
    tail: Coord,
    positions: Vec<Coord>,
}

impl HeadTail {
    fn new() -> Self {
        HeadTail {
            head: Coord::new(),
            tail: Coord::new(),
            positions: vec![],
        }
    }

    fn move_head(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Right(amount) => {
                for _ in 0..amount {
                    self.head.x += 1;
                    self.tail.approach(&self.head);
                    self.positions.push(self.tail);
                }
            }
            Instruction::Left(amount) => {
                for _ in 0..amount {
                    self.head.x -= 1;
                    self.tail.approach(&self.head);
                    self.positions.push(self.tail);
                }
            }
            Instruction::Up(amount) => {
                for _ in 0..amount {
                    self.head.y += 1;
                    self.tail.approach(&self.head);
                    self.positions.push(self.tail);
                }
            }
            Instruction::Down(amount) => {
                for _ in 0..amount {
                    self.head.y -= 1;
                    self.tail.approach(&self.head);
                    self.positions.push(self.tail);
                }
            }
        }
    }

    // fn _distance_between(&self) -> u8 {
    //     let x_dist: u8 = self.head.x.abs_diff(self.tail.x);
    //     let y_dist: u8 = self.head.x.abs_diff(self.tail.y);

    //     u8::max(x_dist, y_dist)
    // }

    fn number_unique_tail_positions(&self) -> usize {
        let unique = self
            .positions
            .clone()
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter();

        unique.len()
    }
}

#[derive(Debug)]
enum Instruction {
    Right(u8),
    Left(u8),
    Up(u8),
    Down(u8),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        if let Some(direction) = split.next() {
            let steps = split
                .next()
                .expect("Has valid input")
                .parse()
                .expect("Has valid input");

            match direction {
                "R" => return Ok(Instruction::Right(steps)),
                "L" => return Ok(Instruction::Left(steps)),
                "U" => return Ok(Instruction::Up(steps)),
                "D" => return Ok(Instruction::Down(steps)),
                _ => unreachable!(),
            };
        }

        Err("Invalid input".into())
    }
}

pub async fn run() {
    let data = get_day_data(9, 2022).await;
    let test_data = get_day_test_data(9, 2022);

    fn parse(d: &str) -> HeadTail {
        let mut head_tail = HeadTail::new();

        d.lines().for_each(|line| {
            let instruction: Instruction = line.parse().expect("Input is parsable");
            head_tail.move_head(instruction);
        });

        head_tail
    }

    fn part_one(d: &str) -> String {
        let head_tail = parse(d);
        head_tail.number_unique_tail_positions().to_string()
    }

    fn part_two(_d: &str) -> String {
        // parse(d);
        String::default()
    }

    check_results((part_one(&test_data), "13"), (part_two(&test_data), ""));

    println!("Tests passed");

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(9, 2022, &part_one, &part_two, started);

    check_results((part_one, "6256"), (part_two, ""));
}
