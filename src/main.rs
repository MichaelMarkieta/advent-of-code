use std::env;
use std::collections::HashMap;
use reqwest;

fn d01p01(expense_report: &Vec<&str>) {
    let answer1: i32;
    let mut expense_report_hashmap: HashMap<i32, i32> = HashMap::new();
    for itm in expense_report.iter() {
        let itm_int = itm.parse::<i32>().unwrap();
        let itm_remainder = 2020 - itm_int;
        if !expense_report_hashmap.contains_key(&itm_remainder) {
            expense_report_hashmap.insert(itm_int, itm_remainder);
        } else {
            println!("We found a pair of values that add up to 2020: {} and {}", itm_int, itm_remainder);
            answer1 = itm_int * itm_remainder;
            println!("Multiply them together and get: {}", answer1);
            return
        }
    }
}

fn d01p02(expense_report: &Vec<&str>) {
    let answer2: i32;
    for itm_a in expense_report.iter() {
        let itm_a_int = itm_a.parse::<i32>().unwrap();
        for itm_b in expense_report.iter() {
            let itm_b_int = itm_b.parse::<i32>().unwrap();
            for itm_c in expense_report.iter() {
                let itm_c_int = itm_c.parse::<i32>().unwrap();
                if itm_a_int + itm_b_int + itm_c_int == 2020 {
                    println!("We found three values that add up to 2020: {}, {} and {}", itm_a_int, itm_b_int, itm_c_int);
                    answer2 = itm_a_int * itm_b_int * itm_c_int;
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

    let expense_report = response.trim_end().split("\n").collect::<Vec<&str>>();

    d01p01(&expense_report);
    d01p02(&expense_report);

    Ok(())
}