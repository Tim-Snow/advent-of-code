use std::{
    cell::RefCell,
    ops::Sub,
    rc::{Rc, Weak},
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

    fn set_parent(&self, parent: Weak<Directory>) {
        *self.parent.borrow_mut() = parent;
    }

    fn add_child(&self, child: &Rc<Directory>) {
        self.children.borrow_mut().push(child.clone());
    }

    fn add_file_size(&self, size: u32) {
        let mut file_sizes = vec![];

        for s in self.file_sizes.borrow().iter() {
            file_sizes.push(*s);
        }

        file_sizes.push(size);

        *self.file_sizes.borrow_mut() = file_sizes;
    }

    fn get_size(&self) -> u32 {
        let mut size = 0;

        for child in self.children.borrow().iter() {
            size += child.get_size();
        }

        for file in self.file_sizes.borrow().iter() {
            size += file;
        }

        size
    }
}

pub async fn run() {
    let data = get_day_data(7, 2022).await;
    let test_data = get_day_test_data(7, 2022);

    fn parse(d: &str) -> Directory {
        let root = Rc::new(Directory::new("root".into()));
        let mut current = Rc::clone(&root);

        d.lines().for_each(|line| {
            if line.starts_with('$') {
                if line.eq("$ cd /") || line.eq("ls") {
                    return;
                }

                if line.eq("$ cd ..") {
                    current = current.get_parent().unwrap();
                    return;
                }

                if line.starts_with("$ cd") {
                    let name = line.split_whitespace().last().unwrap();
                    let child = Rc::new(Directory::new(name.into()));

                    child.set_parent(Rc::downgrade(&current));
                    current.add_child(&child);

                    current = child;
                }
            } else {
                match line.split_whitespace().next().unwrap().parse::<u32>() {
                    Err(_) => {}
                    Ok(size) => {
                        current.add_file_size(size);
                    }
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
        let total_space = 70000000;
        let required_space = 30000000;

        let used = root.get_size();
        let unused = total_space.sub(&used);
        let to_free = required_space - unused;

        fn find_size_of_directories_greater_than(size: &u32, dir: &Directory) -> Vec<u32> {
            let mut accum = vec![];

            if dir.get_size().gt(size) {
                accum.push(dir.get_size());
            }

            for child in dir.children.borrow().iter() {
                if child.get_size().gt(size) {
                    accum.push(child.get_size());
                }

                accum.append(&mut find_size_of_directories_greater_than(size, child));
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
