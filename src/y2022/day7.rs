use std::{
    cell::RefCell,
    ops::{AddAssign, Sub},
    rc::{Rc, Weak},
    str::FromStr,
    time::Instant,
};

use crate::util::{check_results, get_day_data, get_day_test_data, log_results};

#[derive(Debug)]
struct Directory {
    _name: String,
    parent: RefCell<Weak<Directory>>,
    children: RefCell<Vec<Rc<Directory>>>,
    file_sizes: RefCell<Vec<u32>>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            _name: name,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            file_sizes: RefCell::new(vec![]),
        }
    }

    fn get_parent(&self) -> Option<Rc<Directory>> {
        self.parent.borrow().upgrade()
    }

    fn set_parent(&self, parent: &Rc<Directory>) {
        *self.parent.borrow_mut() = Rc::downgrade(parent);
    }

    fn add_child(&self, child: &Rc<Directory>) {
        self.children.borrow_mut().push(child.clone());
    }

    fn add_file_size(&self, size: u32) {
        self.file_sizes.borrow_mut().push(size);
    }

    fn get_size(&self) -> u32 {
        let mut size = 0;

        for child in self.children.borrow().iter() {
            size.add_assign(child.get_size());
        }

        for file in self.file_sizes.borrow().iter() {
            size.add_assign(file);
        }

        size
    }
}

enum Line {
    NoOp,
    CdParent,
    Cd(String),
    LsFileSize(u32),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ") {
            if s.eq("$ ls") || s.eq("$ cd /") {
                return Ok(Line::NoOp);
            }
            if s.eq("$ cd ..") {
                return Ok(Line::CdParent);
            }
            if s.starts_with("$ cd") {
                let to_dir = s.split_whitespace().last().unwrap();
                return Ok(Line::Cd(to_dir.into()));
            }
        }

        if let Ok(size) = s.split_whitespace().next().unwrap().parse::<u32>() {
            return Ok(Line::LsFileSize(size));
        }

        Err("Not a command".into())
    }
}

pub async fn run() {
    let data = get_day_data(7, 2022).await;
    let test_data = get_day_test_data(7, 2022);

    fn parse(d: &str) -> Directory {
        let root = Rc::new(Directory::new("root".into()));
        let mut current = Rc::clone(&root);

        d.lines().for_each(|line| {
            if let Ok(command) = line.parse::<Line>() {
                match command {
                    Line::LsFileSize(size) => current.add_file_size(size),
                    Line::CdParent => current = current.get_parent().unwrap(),
                    Line::Cd(dir) => {
                        let child = Rc::new(Directory::new(dir));

                        child.set_parent(&current);
                        current.add_child(&child);

                        current = child;
                    }
                    Line::NoOp => {}
                }
            }
        });

        Rc::try_unwrap(root).unwrap()
    }

    fn part_one(d: &str) -> String {
        let root = parse(d);

        fn sum_directories_smaller_than(size: u32, dir: &Directory) -> u32 {
            let mut accum = vec![];

            if dir.get_size().lt(&size) {
                accum.push(dir.get_size());
            }

            for child in dir.children.borrow().iter() {
                accum.push(sum_directories_smaller_than(size, child));
            }

            accum.iter().sum()
        }

        sum_directories_smaller_than(100000, &root).to_string()
    }

    fn part_two(d: &str) -> String {
        let root = parse(d);
        let total_space = 70_000_000;
        let required_space = 30_000_000;

        let used = root.get_size();
        let to_free = required_space.sub(total_space.sub(used));

        fn find_size_of_directories_greater_than(size: &u32, dir: &Directory) -> Vec<u32> {
            let mut accum = vec![];

            if dir.get_size().gt(size) {
                accum.push(dir.get_size());

                for child in dir.children.borrow().iter() {
                    if child.get_size().gt(size) {
                        accum.push(child.get_size());
                        accum.append(&mut find_size_of_directories_greater_than(size, child));
                    }
                }
            }

            accum
        }

        let mut sizes = find_size_of_directories_greater_than(&to_free, &root);

        sizes.sort();

        sizes.first().unwrap().to_string()
    }

    check_results(
        (part_one(&test_data), "95437"),
        (part_two(&test_data), "24933642"),
    );

    let started = Instant::now();

    let part_one = part_one(&data);
    let part_two = part_two(&data);

    log_results(7, 2022, &part_one, &part_two, started);

    check_results((part_one, "1141028"), (part_two, "8278005"));
}
