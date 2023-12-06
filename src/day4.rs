use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn process_input_lines(haystack: &str) -> i32 {
    haystack.lines().map(|line| process_input_line(line)).sum()
}

fn process_input_line(haystack: &str) -> i32 {
    let parts: Vec<&str> = haystack[haystack.find(":").unwrap()..].split('|').collect();
    let my_nums: HashSet<i32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let win_nums: HashSet<i32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let i: HashSet<_> = my_nums.intersection(&win_nums).cloned().collect();

    if i.len() > 0 {
        1 << (i.len() - 1)
    } else {
        0
    }
}

pub fn point_value() {
    let mut contents = String::new();

    match File::open("data/d4_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => println!("Puzzle # 4.1: {}", process_input_lines(&contents)),
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
