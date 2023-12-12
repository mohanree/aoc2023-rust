/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn generate_combinations(s: &str) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back(s.to_string());

    let mut r = Vec::new();
    while let Some(current) = queue.pop_front() {
        if let Some(pos) = current.find('?') {
            for c in ['#', '.'] {
                let mut new_str = current.clone();
                new_str.replace_range(pos..pos + 1, &c.to_string());
                queue.push_back(new_str);
            }
        } else {
            r.push(current);
        }
    }

    r
}

fn process_input_line(line: &str) -> usize {
    let v: Vec<&str> = line.split_whitespace().collect();

    let target_run_lengths: Vec<usize> = v[1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    
    generate_combinations(v[0])
        .iter().filter(|e|{
            let runs: Vec<usize> = e
            .split('.')
            .filter(|x| x.len() > 0)
            .map(|y| y.len())
            .collect();
            target_run_lengths == runs 
        }
    ).count()
}

fn process_input_lines(haystack: &str) -> usize {
    haystack.lines().map(|line| process_input_line(line)).sum()
}

fn process_input_lines2(haystack: &str) -> usize {
    0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d12_input.txt") {
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
