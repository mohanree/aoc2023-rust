/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn process_input_line(line: &str) -> usize {

    let v: Vec<&str> = line.split_whitespace().collect();

    let t: Vec<i32> = v[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();


    0
}

fn process_input_lines(haystack: &str) -> usize {
     haystack
        .lines()
        .map(|line| process_input_line(line))
        .sum();    
}

fn process_input_lines2(haystack: &str) -> usize {
    0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d11_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 11.1: {}", process_input_lines(&contents));
                println!("Puzzle # 11.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
