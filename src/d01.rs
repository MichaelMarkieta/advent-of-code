use std::collections::HashMap;

fn d01p01(expense_report: &Vec<i32>) {
    let answer: i32;
    let mut expense_report_hashmap: HashMap<i32, i32> = HashMap::new();
    for itm in expense_report.iter() {
        let itm_remainder = 2020 - itm;
        if !expense_report_hashmap.contains_key(&itm_remainder) {
            expense_report_hashmap.insert(*itm, itm_remainder);
        } else {
            println!(
                "We found a pair of values that add up to 2020: {} and {}",
                itm,
                itm_remainder
            );
            answer = itm * itm_remainder;
            println!("Answer to d01p01: {}", answer);
            return;
        }
    }
}

fn d01p02(expense_report: &Vec<i32>) {
    let answer: i32;
    for itm_a in expense_report.iter() {
        for itm_b in expense_report.iter() {
            for itm_c in expense_report.iter() {
                if itm_a + itm_b + itm_c == 2020 {
                    println!(
                        "We found three values that add up to 2020: {}, {} and {}",
                        itm_a,
                        itm_b,
                        itm_c
                    );
                    answer = itm_a * itm_b * itm_c;
                    println!("Answer to d01p02: {}", answer);
                    return;
                }
            }
        }
    }
}

pub fn d01(data: Vec<String>) {
    let expense_report: Vec<i32> = data.into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    d01p01(&expense_report);
    d01p02(&expense_report);
}
