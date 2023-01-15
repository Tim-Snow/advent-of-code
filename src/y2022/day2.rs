use std::time::Instant;

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn new(str: &str) -> Move {
        match str {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(m: &Move) -> u16 {
        match m {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(result: &Outcome, used: &Move) -> u16 {
        let res: u16 = match result {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };

        res + Move::score(used)
    }

    fn new(str: &str) -> Outcome {
        match str {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

struct Line {}

impl Line {
    fn eval(from: Move, to: &Move) -> Outcome {
        match from {
            Move::Rock => match to {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Lose,
            },
            Move::Paper => match to {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Win,
            },
            Move::Scissors => match to {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Draw,
            },
        }
    }
}

pub async fn run() {
    let data = get_day_data(2, 2022).await;
    let test_data = get_day_test_data(2, 2022);

    fn part_one(d: &str) -> u16 {
        let mut total: u16 = 0;

        d.lines().for_each(|line| {
            let mut moves = line.split_whitespace();
            let opponent_move = Move::new(moves.next().unwrap());
            let me_move = Move::new(moves.next().unwrap());
            let result = Line::eval(opponent_move, &me_move);

            total += Outcome::score(&result, &me_move);
        });

        total
    }

    fn part_two(d: &str) -> u16 {
        let mut total: u16 = 0;

        d.lines().for_each(|line| {
            let mut moves = line.split_whitespace();
            let opponent_move = Move::new(moves.next().unwrap());
            let outcome = Outcome::new(moves.next().unwrap());

            let me_move = match (&opponent_move, &outcome) {
                (Move::Rock, Outcome::Draw) => Move::Rock,
                (Move::Rock, Outcome::Win) => Move::Paper,
                (Move::Rock, Outcome::Lose) => Move::Scissors,
                (Move::Paper, Outcome::Draw) => Move::Paper,
                (Move::Paper, Outcome::Win) => Move::Scissors,
                (Move::Paper, Outcome::Lose) => Move::Rock,
                (Move::Scissors, Outcome::Draw) => Move::Scissors,
                (Move::Scissors, Outcome::Win) => Move::Rock,
                (Move::Scissors, Outcome::Lose) => Move::Paper,
            };

            total += Outcome::score(&outcome, &me_move);
        });

        total
    }

    check_results(
        (part_one(&test_data).to_string(), "15"),
        (part_two(&test_data).to_string(), "12"),
    );

    let started = Instant::now();

    let part_one = part_one(&data).to_string();
    let part_two = part_two(&data).to_string();

    log_results(2, 2022, &part_one, &part_two, started);

    check_results((part_one, "13221"), (part_two, "13131"));
}
