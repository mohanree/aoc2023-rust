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

fn process_input_lines2(block: &str) -> usize {
    let mut boxes: [Vec<(&str, &str)>; 256] = std::array::from_fn(|_| Vec::new());

    let hash = |e : &str| {
        e.chars().fold(0, |acc, c| {
            (acc + c as u64) * 17 % 256
        })
    };

    for l in block.lines() {
        for s in l.split(",") {
            if s.contains("=") {
                let mut parts = s.split("=");
                let label = parts.next().unwrap_or_default();
                if let Some(value) = parts.next() {
                    let idx = hash(label) as usize;
                    let lens_box = &mut boxes[idx];
                    let lens_position = lens_box.iter().position(|lens| lens.0 == label);
                    match lens_position {
                        Some(pos) => lens_box[pos] = (label, value), 
                        None => lens_box.push((label, value)),  
                    }
                }
            }
            else {
                let label = &s[..s.len() - 1];
                let idx = hash(label) as usize;
                let lens_box = &mut boxes[idx];

                let lens_position = lens_box.iter().position(|lens| lens.0 == label);
                if let Some(pos) = lens_position {
                     lens_box.remove(pos);
                }
            }
        }
    }

    let mut power: usize = 0;
    for (i, lens) in boxes.iter().enumerate() {
        for (j, len) in lens.iter().enumerate() {
            power += (i+1) * (j+1) * len.1.parse::<usize>().unwrap();
        }
    }

    println!("{:?}", boxes);

    power
}


pub fn play() {
    let mut contents = String::new();

    match File::open("data/d15_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 15.1: {}", process_input_lines(&contents));
                println!("Puzzle # 15.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
