use std::env;
use std::collections::HashMap;
use reqwest;

fn d01p01(expense_report: &Vec<i32>) {
    let answer1: i32;
    let mut expense_report_hashmap: HashMap<i32, i32> = HashMap::new();
    for itm in expense_report.iter() {
        let itm_remainder = 2020 - itm;
        if !expense_report_hashmap.contains_key(&itm_remainder) {
            expense_report_hashmap.insert(*itm, itm_remainder);
        } else {
            println!("We found a pair of values that add up to 2020: {} and {}", itm, itm_remainder);
            answer1 = itm * itm_remainder;
            println!("Multiply them together and get: {}", answer1);
            return
        }
    }
}

fn d01p02(expense_report: &Vec<i32>) {
    let answer2: i32;
    for itm_a in expense_report.iter() {
        for itm_b in expense_report.iter() {
            for itm_c in expense_report.iter() {
                if itm_a + itm_b + itm_c == 2020 {
                    println!("We found three values that add up to 2020: {}, {} and {}", itm_a, itm_b, itm_c);
                    answer2 = itm_a * itm_b * itm_c;
                    println!("Multiply them together and get: {}", answer2);
                    return
                }
            }   
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let aoc_session_id = env::var("AOC_SESSION_ID").unwrap_or("none".to_string());

    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://adventofcode.com/2020/day/1/input")
        .header("cookie", format!("session={}", aoc_session_id))
        .send()
        .unwrap()
        .text()
        .unwrap();

    let expense_report = response.trim_end().split("\n").map(|x| x.parse::<i32>().unwrap()).collect();

    d01p01(&expense_report);
    d01p02(&expense_report);

    Ok(())
}