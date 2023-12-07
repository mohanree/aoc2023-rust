use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn process_input_lines(haystack: &str) -> i32 {
    haystack
        .lines()
        .map(|line| process_input_line(line).1)
        .sum()
}

fn process_input_lines_p2(haystack: &str) -> i32 {
    let v: Vec<usize> = haystack.lines().map(|x| process_input_line2(x)).collect();

    let mut c = vec![0; v.len()];
    for i in 0..v.len() {
        c[i] = c[i] + 1;
        for j in (i + 1)..(i + 1 + v[i] as usize) {
            if j < c.len() {
                c[j] = c[j] + c[i];
            }
        }
    }

    //println!("{:?}", c);
    c.iter().sum()
}

fn process_input_line2(haystack: &str) -> usize {
    let t = haystack.find(":").unwrap();
    let id = &haystack[..t];
    let parts: Vec<&str> = haystack[t..].split('|').collect();
    let my_nums: HashSet<i32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let win_nums: HashSet<i32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let i: HashSet<_> = my_nums.intersection(&win_nums).cloned().collect();

    return i.len();
}

fn process_input_line(haystack: &str) -> (&str, i32) {
    let t = haystack.find(":").unwrap();
    let id = &haystack[..t];
    let parts: Vec<&str> = haystack[t..].split('|').collect();
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
        //println!("{:?}", (id, 1 << (i.len() - 1)));
        (id, 1 << (i.len() - 1))
    } else {
        (id, 0)
    }
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d4_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 4.1: {}", process_input_lines(&contents));
                println!("Puzzle # 4.2: {}", process_input_lines_p2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
