use colored::Colorize;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::time::Instant;

pub async fn get_day_data(day: u16, year: u16) -> String {
    let path = format!("res/{day}.txt");
    let file = File::open(&path);

    match file {
        Ok(_) => read_to_string(path).unwrap(),
        Err(e) => {
            println!("File does not exist: {e}");

            let content = get_client()
                .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
                .send()
                .unwrap()
                .text()
                .unwrap();

            let mut output = File::create(path).unwrap();

            write!(output, "{}", content).unwrap();

            content
        }
    }
}

fn get_client() -> Client {
    dotenv::dotenv().ok();

    let session = format!("session={}", dotenv::var("COOKIE").unwrap());

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(session.as_str()).unwrap());
    headers.insert(
        "User-Agent",
        HeaderValue::from_str(dotenv::var("USER_AGENT").unwrap().as_str()).unwrap(),
    );

    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn log_result(day: u8, year: u16, pt1: &str, pt2: &str, started_at: Instant) {
    println!(
        "\n{}\nPart 1: {}\nPart 2: {}\nExecution time: {}",
        format!("Year {year} - Day {day}").blue().bold().underline(),
        pt1.green(),
        pt2.green(),
        format!("{:?}", started_at.elapsed()).yellow().bold()
    );
}
