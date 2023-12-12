/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn generate_combinations(s: &str) -> Vec<String> {
    let mut r = vec![s.to_string()];

    let mut i = 0;
    while i < r.len() {
        if let Some(pos) = r[i].find('?') {
            let mut v: Vec<String> = vec![];
            for c in ['#', '.'] {
                let mut t = r[i].clone();
                t.replace_range(pos..pos + 1, &c.to_string());
                v.push(t);
            }
            r.remove(i);
            r.append(&mut v);
            continue;
        }
        i += 1;
    }

    r
}

fn process_input_line(line: &str) -> usize {
    let v: Vec<&str> = line.split_whitespace().collect();

    let t: Vec<usize> = v[1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let possible_conds = generate_combinations(v[0]);

    let mut count = 0;
    possible_conds.iter().for_each(|e| {
        let runs: Vec<usize> = e
            .split('.')
            .filter(|x| x.len() > 0)
            .map(|y| y.len())
            .collect();
        //println!("{:?} =? {:?}", runs, t);
        if t == runs {
            count += 1;
        }
    });

    count
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
