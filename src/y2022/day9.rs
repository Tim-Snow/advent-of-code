use std::{
    collections::HashSet,
    ops::{AddAssign, Div, Sub},
    str::FromStr,
    time::Instant,
};

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

    fn add(&mut self, x: i16, y: i16) -> Self {
        self.x.add_assign(x);
        self.y.add_assign(y);

        *self
    }

    fn approach(&mut self, other: &Coord) -> Self {
        let (dx, dy) = match (self.x.abs_diff(other.x), self.y.abs_diff(other.y)) {
            (0, 2) => (0, (other.y.sub(self.y)).div(2)),
            (2, 0) => ((other.x.sub(self.x)).div(2), 0),
            (1, 2) => ((other.x.sub(self.x)), (other.y.sub(self.y)).div(2)),
            (2, 1) => ((other.x.sub(self.x)).div(2), (other.y.sub(self.y))),
            (2, 2) => ((other.x.sub(self.x)).div(2), (other.y.sub(self.y)).div(2)),
            _ => (0, 0),
        };

        self.add(dx, dy);

        *self
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Coord>,
    history: Vec<Coord>,
}

impl Rope {
    fn new(length: u8) -> Self {
        Rope {
            knots: vec![Coord::new(); length.into()],
            history: vec![],
        }
    }

    fn update(&mut self, instruction: Instruction) {
        let (dx, dy, amount) = match instruction {
            Instruction::Right(amount) => (1, 0, amount),
            Instruction::Left(amount) => (-1, 0, amount),
            Instruction::Up(amount) => (0, 1, amount),
            Instruction::Down(amount) => (0, -1, amount),
        };

        for _ in 0..amount {
            let mut next_position = self.knots.first_mut().unwrap().add(dx, dy);

            for knot in self.knots[1..].iter_mut() {
                let new_position = knot.approach(&next_position);

                *knot = new_position;
                next_position = new_position;
            }

            self.history.push(*self.knots.last().unwrap());
        }
    }

    fn number_unique_tail_positions(&self) -> usize {
        self.history
            .clone()
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .len()
    }
}

#[derive(Debug, Clone, Copy)]
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
    let data = parse(get_day_data(9, 2022).await);
    let test_data = parse(get_day_test_data(9, 2022));

    fn parse(d: String) -> Vec<Instruction> {
        d.lines()
            .map(|line| line.parse().expect("Input is parsable"))
            .collect()
    }

    fn simulate(length: u8, instructions: &Vec<Instruction>) -> String {
        let mut rope = Rope::new(length);

        for instruction in instructions {
            rope.update(*instruction);
        }

        rope.number_unique_tail_positions().to_string()
    }

    fn part_one(instructions: &Vec<Instruction>) -> String {
        simulate(2, instructions)
    }

    fn part_two(instructions: &Vec<Instruction>) -> String {
        simulate(10, instructions)
    }

    check_results((part_one(&test_data), "88"), (part_two(&test_data), "36"));

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(9, 2022, &part_one, &part_two, started);

    check_results((part_one, "6256"), (part_two, "2665"));
}
