fn d03p01(map: &Vec<String>) {
    let mut encountered: Vec<String> = Vec::new();
    let mut x: i32 = 0;
    for row in map {
        match row.chars().nth(x as usize) {
            Some(v) => encountered.push(v.to_string()),
            None => continue,
        }
        x += 3;
    }
    let count_trees_encountered = encountered.iter().filter(|&s| *s == "#").count();
    println!("Answer to d03p01: {}", count_trees_encountered);
}


pub fn d03(data: Vec<String>) {
    let line_length = data[0].len() as f32;
    let line_length_dividend = (line_length / 3.0) as f32;
    let num_lines = data.len() as f32;
    let line_length_factor = ((num_lines / line_length_dividend) as f32).ceil() as usize;

    let mut map: Vec<String> = Vec::new();
    for line in data {
        let repeated_line = line.repeat(line_length_factor);
        map.push(repeated_line);
    }

    d03p01(&map);
}
