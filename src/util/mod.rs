use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::time::Instant;

#[cfg(windows)]
pub static LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub static LINE_ENDING: &str = "\n";

pub async fn get_day_data(day: u8, year: u16) -> String {
    let path = format!("res/{year}/{day}.txt");
    let file = File::open(&path);

    match file {
        Ok(_) => read_to_string(path).unwrap(),
        Err(e) => {
            println!("File does not exist: {e}");
            println!("Fetching...");

            let content = get_client()
                .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let mut output = File::create(&path).unwrap();

            println!("Writing fetched data to: {}", &path);
            write!(output, "{}", content).unwrap();

            content
        }
    }
}

pub fn get_day_test_data(day: u8, year: u16) -> String {
    read_to_string(format!("res/{year}/{day}.test.txt")).expect("Test data has not been created")
}

fn get_client() -> Client {
    let session = format!("session={}", dotenv::var("COOKIE").unwrap());

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(session.as_str()).unwrap());
    headers.insert(
        "User-Agent",
        HeaderValue::from_str(dotenv::var("USER_AGENT").unwrap().as_str()).unwrap(),
    );

    Client::builder().default_headers(headers).build().unwrap()
}

pub fn log_result(day: u8, year: u16, pt1: &str, pt2: &str, started_at: Instant) {
    println!(
        "\n{}\nPart 1: {}\nPart 2: {}\nExecution time: {}",
        format!("{year} - Day {day}").blue().bold().underline(),
        pt1.green(),
        pt2.green(),
        format!("{:?}", started_at.elapsed()).yellow().bold()
    );
}
