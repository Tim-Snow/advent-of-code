use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use crate::util::get_day_data;

#[derive(Debug)]
enum Line {
    Noop,
    AddX(i16),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace();
        let count = split.to_owned().count();

        match count {
            1 => Ok(Line::Noop),
            2 => Ok(Line::AddX(split.last().unwrap().parse().unwrap())),
            _ => Err(String::from("What")),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self)
        match self {
            Line::Noop => write!(f, "No-op"),
            Line::AddX(value) => write!(f, "Add X: {value}"),
        }
    }
}

#[derive(Debug)]
struct Pending {
    value: i16,
    time: u8,
}

impl Pending {
    fn new(value: i16) -> Self {
        Pending { value, time: 1 }
    }

    fn progress(&mut self) -> i16 {
        match self.time {
            0 => 0,
            1 => {
                self.time = self.time.sub(1);
                self.value
            }
            _ => unreachable!(),
        }
    }
}

pub async fn run() {
    // let test_data = read_to_string("res/10.txt.test").unwrap();
    let data = get_day_data(10, 2022).await;

    fn part_one(d: &str) -> String {
        let mut strength = 1;
        let mut strengths: Vec<i16> = vec![];
        let mut cycle = 0;
        let num_signal_strengths = 6;
        let signal_strength_interval: u16 = 40;
        let initial_signal_strength: u16 = 20;
        let mut pending: Vec<Pending> = vec![];

        d.lines()
            .map(|l| l.parse::<Line>().unwrap())
            .for_each(|line| {
                match line {
                    Line::Noop => pending.iter_mut().for_each(|p| {
                        strength = strength.add(p.progress());
                        cycle = cycle.add(1);
                    }),
                    Line::AddX(value) => {
                        pending.iter_mut().for_each(|p| {
                            strength = strength.add(p.progress());
                            cycle = cycle.add(1);
                        });
                        pending.push(Pending::new(value));
                        pending.iter_mut().for_each(|p| {
                            strength = strength.add(p.progress());
                            cycle = cycle.add(1);
                        })
                    }
                };
                pending.retain(|p| p.time.ne(&0));
                // needs to run each time p progresses
                // if cycle.eq(&initial_signal_strength)
                //     || cycle.gt(&initial_signal_strength)
                //         && (cycle.sub(&initial_signal_strength) % signal_strength_interval).eq(&0)
                // {
                //     dbg!("{} {}", cycle, strength);
                //     strengths.push((cycle as i16).mul(strength));
                //     strength = 1;
                // }
            });

        for i in
            cycle..initial_signal_strength.add(num_signal_strengths.mul(signal_strength_interval))
        {
            if !pending.is_empty() {
                pending
                    .iter_mut()
                    .for_each(|p| strength = strength.add(p.progress()));
                pending.retain(|p| p.time.ne(&0));
            }
            if i.eq(&initial_signal_strength)
                || i.gt(&initial_signal_strength)
                    && (i.sub(&initial_signal_strength) % signal_strength_interval).eq(&0)
            {
                strengths.push((i as i16).mul(strength));
                strength = 1;
            }
        }

        dbg!("{}", strengths);

        String::default()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    println!("1: {}\n2: {}", part_one(&data), part_two(&data))
}
