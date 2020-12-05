use std::env;
use reqwest;

pub mod d01;
pub mod d02;

use d01::d01;

fn get_data(day: i32) -> Vec<String> {
    let aoc_session_id = env::var("AOC_SESSION_ID").unwrap_or("none".to_string());
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2020/day/{}/input", day);
    let response = client
        .get(&url)
        .header("cookie", format!("session={}", aoc_session_id))
        .send()
        .unwrap()
        .text()
        .unwrap();
    let data: Vec<String> = response.trim_end().split("\n").map(|s| s.to_string()).collect();
    return data
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse::<i32>().unwrap();
    let data = get_data(day);
    match day {
        1 => d01(data),
        _ => println!("Not ready for that day yet!"),
    }
}