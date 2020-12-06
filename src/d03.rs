pub fn d03(data: Vec<String>) {
    let num_lines = data.len();
    let line_length_required = (num_lines / 3) as f32; // we move across 3 for every line in the input
    let line_length = data[0].len() as f32;
    let line_length_factor = ((line_length_required / line_length) as f32).ceil() as usize; // extend the lines by this factor to have enough data
    
    let mut map: Vec<String> = Vec::new();
    for line in data {
        let repeated_line = line.repeat(line_length_factor);
        map.push(repeated_line);
    }
}