#[derive(Debug)]
struct Policy {
    password: String,
    min: u16,
    max: u16,
    char: String,
}

fn d02p01(policies: &Vec<Policy>) {
    let mut good_passwords = 0;
    let mut _bad_passwords = 0;
    for policy in policies {
        let min = policy.min;
        let max = policy.max;
        let count = policy.password.matches(&policy.char).count() as u16;
        if count >= min && count <= max {
            good_passwords += 1;
        } else {
            continue
        }
    }
    println!("Answer to d02p01: {}", good_passwords);
}

pub fn d02(data: Vec<String>) {  
    let mut passwords: Vec<Policy> = Vec::new(); // [{password:p,min:x,max:y,char:z,}]

    for line in data {
        let line_split = line.split(":").map(|s| s.trim_start().trim_end().to_string()).collect::<Vec<String>>();
        let policy = &line_split[0];
        let password = &line_split[1];
        let policy_parts: Vec<&str> = policy.split_whitespace().collect();
        let policy_lengths: Vec<u16> = policy_parts[0].split("-").map(|s| s.parse().unwrap()).collect::<Vec<u16>>();
        let policy_lengths_min: u16 = policy_lengths[0];
        let policy_lengths_max: u16 = policy_lengths[1];
        let policy_char = policy_parts[1];
        
        passwords.push(Policy{
            password: password.to_string(),
            min: policy_lengths_min,
            max: policy_lengths_max,
            char: policy_char.to_string(),
        });
    }

    d02p01(&passwords);
}