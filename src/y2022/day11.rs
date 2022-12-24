use std::str::FromStr;

use crate::util::get_day_data;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u16>,
    operation: String,
    test: String,
    if_true_monkey: usize,
    if_false_monkey: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(lines: &str) -> Result<Self, String> {
        let mut lines = lines.lines();

        let id = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .replace(":", "")
            .parse()
            .unwrap();

        let items: Vec<u16> = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|c| c.trim().parse().unwrap())
            .collect();

        let operation = lines
            .next()
            .unwrap()
            .split("new = old ")
            .last()
            .unwrap()
            .trim()
            .to_string();

        let test = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .to_string();

        let if_true_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let if_false_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            id,
            items,
            operation,
            test,
            if_true_monkey,
            if_false_monkey,
        })
    }
}

pub async fn run() {
    let data = get_day_data(11, 2022).await;

    fn parse(d: &str) -> Vec<Monkey> {
        d.split("\n\n")
            .map(|monkey| monkey.parse().unwrap())
            .collect()
    }

    fn part_one(d: &str) -> String {
        let mut monkeys = parse(d);

        let move_list = monkeys.iter_mut().map(|monkey| {
            monkey.items.iter_mut().map(|item| -> (u16, usize) {
                let mut operation_split = monkey.operation.split_whitespace();
                let operator = operation_split.next().unwrap();
                let amount = operation_split.next().unwrap();
                let amount: u16 = amount.parse().unwrap_or(*item);

                *item = match operator {
                    "+" => item.checked_add(amount).unwrap(),
                    "-" => item.checked_sub(amount).unwrap(),
                    "*" => item.checked_mul(amount).unwrap(),
                    "%" => item.checked_div(amount).unwrap(),
                    _ => unreachable!(),
                };

                *item = *item / 3;

                let mut test_split = monkey.test.split_whitespace();

                assert_eq!(test_split.next().unwrap(), "divisible");

                let test_value: u16 = test_split.last().unwrap().parse().unwrap();

                if *item % test_value == 0 {
                    //monkeys.iter().find(|other_monkey| other_monkey.id == monkey.if_true_monkey).unwrap();
                    println!("divisible {} {}", &item, &test_value);
                    return (*item, monkey.if_true_monkey);
                } else {
                    println!("not divisible {} {}", &item, &test_value);
                    return (*item, monkey.if_false_monkey);
                }
            })
        });

        // move_list.for_each(|(item, id)| {
        //     dbg!(item, id);
        // });

        dbg!(monkeys);

        String::default()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    println!("1: {}\n2: {}", part_one(&data), part_two(&data))
}
