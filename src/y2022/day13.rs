use std::{fs::read_to_string, str::FromStr, time::Instant};

use crate::util::{get_day_data, log_result};

#[derive(Debug)]
enum IntList {
    IntVector(Vec<u8>),
    IntVectorVector(Vec<Box<IntList>>),
}

// impl<T> FromIterator<T> for IntList
// where
//     T: Into<IntList>,
// {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         let mut vec = Vec::new();
//         for item in iter {
//             vec.push(item.into());
//         }
//         IntList::IntVector(Vec::from(vec))
//         // IntList::IntVectorVector(vec)
//     }
// }

#[derive(Debug)]
struct Pair {
    left: IntList,
    right: IntList,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        // let left = lines
        //     .next()
        //     .unwrap()
        //     .split('[')
        //     .map(|v| v.parse::<u8>().unwrap())
        //     .map(Box::new)
        //     .collect();

        // let right = vec![]; // lines.next();

        // Pair { left, right }
        Ok(Pair {
            left: IntList::IntVector(Vec::new()),
            right: IntList::IntVector(Vec::new()),
        })
    }
}

pub async fn run() {
    let data = get_day_data(13, 2022).await;
    // let test_data = read_to_string("res/13.txt").unwrap();

    fn parse(d: &str) -> Vec<Pair> {
        let pairs = d.split("\n\n");

        // dbg!(pairs.next().unwrap().parse::<Pair>().unwrap());

        // split.map(|s| parse::<Pair>(s)).collect::<Vec<Pair>>();

        vec![]
    }

    fn part_one(d: &str) -> &str {
        ""
    }

    log_result(13, 2022, part_one(&data), "", Instant::now());
}
