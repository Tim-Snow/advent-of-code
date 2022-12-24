use crate::util::get_day_data;

static STARTING_POSITION: char = 'S';
static END_POSITION: char = 'E';

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

pub async fn run() {
    let data = get_day_data(12, 2022).await;
    // let data = read_to_string("res/12.txt").unwrap();

    fn get_coords_for_char(d: &str, c: char) -> Option<Position> {
        d.lines()
            .enumerate()
            .find_map(|(y, line)| line.find(|ch: char| ch.eq(&c)).map(|x| Position { x, y }))
    }

    fn parse(d: &str) -> (Position, Position) {
        let matrix = d.lines().map(|line| line.chars());

        let start_position = get_coords_for_char(d, STARTING_POSITION).unwrap();
        let end_position = get_coords_for_char(d, END_POSITION).unwrap();

        (start_position, end_position)
    }

    fn part_one(d: &str) -> String {
        let (start, end) = parse(d);

        dbg!(&start, &end);
        dbg!(start.x, end.y);

        String::default()
    }

    fn part_two(_d: &str) -> String {
        String::default()
    }

    println!("1: {}\n2: {}", part_one(&data), part_two(&data))
}
