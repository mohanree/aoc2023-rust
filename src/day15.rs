/*
*/

use crate::util::util;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::vec;

fn process_input_block(block: &str) -> u64 {
    block.lines()
        .flat_map(|line| line.split(','))
        .map(|e| {
            e.chars().fold(0, |acc, c| {
                (acc + c as u64) * 17 % 256
            })
        })
        .sum()
}

fn process_input_lines(haystack: &str) -> u64 {
    process_input_block(haystack)
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d15_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 16.1: {}", process_input_lines(&contents));
                //println!("Puzzle # 16.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
