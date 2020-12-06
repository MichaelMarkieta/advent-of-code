use std::collections::HashMap;

fn d02p01(passwords: &HashMap<String, String>) {
    for (policy, password) in passwords {
        let policy_parts: Vec<&str> = policy.split_whitespace().collect();
        let policy_lengths: Vec<&str> = policy_parts[0].split("-").collect();
        let policy_lengths_min: i32 = policy_lengths[0].parse().unwrap();
        let policy_lengths_max: i32 = policy_lengths[1].parse().unwrap();
        let policy_char = policy_parts[1];
        println!("Min: {}, Max: {}, Char: {}, Password: {}", policy_lengths_min, policy_lengths_max, policy_char, password);
    }
}

pub fn d02(data: Vec<String>) {
    let passwords: HashMap<String, String> = data.into_iter()
        .map(|s| s.split(":").map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|vec| (vec[0].to_string(), vec[1].trim_start().to_string()))
        .collect();
    d02p01(&passwords);
}