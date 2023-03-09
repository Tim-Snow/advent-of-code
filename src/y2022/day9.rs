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

    fn approach(&mut self, other: &Coord) -> Self {
        let (dx, dy) = match (self.x.abs_diff(other.x), self.y.abs_diff(other.y)) {
            (2, 0) => ((other.x.sub(self.x)).div(2), 0),
            (2, 1) => ((other.x.sub(self.x)).div(2), (other.y.sub(self.y))),
            (0, 2) => (0, (other.y.sub(self.y)).div(2)),
            (1, 2) => ((other.x.sub(self.x)), (other.y.sub(self.y)).div(2)),
            (2, 2) => ((other.x.sub(self.x)).div(2), (other.y.sub(self.y)).div(2)),
            _ => (0, 0),
        };

        self.x.add_assign(dx);
        self.y.add_assign(dy);

        *self
    }
}

#[derive(Debug)]
struct Rope {
    head: Coord, // TODO: head can just be the first element of tail
    tail: Vec<Coord>,
    positions: Vec<Coord>,
}

impl Rope {
    fn new(length: u8) -> Self {
        Rope {
            head: Coord::new(),
            tail: vec![Coord::new(); length.into()],
            positions: vec![],
        }
    }

    fn move_head(&mut self, instruction: Instruction) {
        let (dx, dy, amount) = match instruction {
            Instruction::Right(amount) => (1, 0, amount),
            Instruction::Left(amount) => (-1, 0, amount),
            Instruction::Up(amount) => (0, 1, amount),
            Instruction::Down(amount) => (0, -1, amount),
        };

        for _ in 0..amount {
            self.head.x += dx;
            self.head.y += dy;
            self.update_tail();
            self.positions.push(*self.tail.last().unwrap());
        }
    }

    fn update_tail(&mut self) {
        let mut new_tail: Vec<Coord> = vec![];
        new_tail.push(self.tail.first_mut().unwrap().approach(&self.head));

        for item in self.tail[1..].iter_mut() {
            new_tail.push(item.approach(new_tail.last().unwrap()));
        }

        self.tail = new_tail;
    }

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

    fn parse(d: &str, length: u8) -> Rope {
        let mut rope = Rope::new(length);

        d.lines().for_each(|line| {
            let instruction: Instruction = line.parse().expect("Input is parsable");
            rope.move_head(instruction);
        });

        rope
    }

    fn part_one(d: &str) -> String {
        let rope = parse(d, 1);
        rope.number_unique_tail_positions().to_string()
    }

    fn part_two(d: &str) -> String {
        let rope = parse(d, 9);
        rope.number_unique_tail_positions().to_string()
    }

    check_results((part_one(&test_data), "88"), (part_two(&test_data), "36"));

    println!("Tests passed");

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(9, 2022, &part_one, &part_two, started);

    check_results((part_one, "6256"), (part_two, "2665"));
}
