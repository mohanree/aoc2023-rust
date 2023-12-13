/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read; 

fn mirror_on_row( grid: &Vec<Vec<char>> ) -> Option<usize> {

    let mut los = usize::MAX;
    for i in 0..grid.len()-1{
        if grid[i] == grid [i+1] { los = i; }
    }

    for j in 0..los {
        let idx = los + j + 1; 
        if idx >= grid.len() {continue;}
        if grid[j] != grid[idx] { return None;}
    }

    Some(los)
}

fn mirror_on_col( grid: &Vec<Vec<char>> ) -> Option<usize> {

    let mut los = usize::MAX;
    if let Some(row_len) = grid.first().map(|row| row.len()) {
        for i in 0..row_len - 1 {
            if grid.iter().all(|row| row.get(i) == row.get(i+1)) {
                los = i;
            }
        }

        for j in 0..los {
            let idx = los + j + 1;
            if idx >= row_len {continue;} 
            if grid.iter().any(|row| row.get(j) != row.get(idx)) {
                return None;
            }
        }
    }

    Some(los)
}

fn process_input_block(block: &str) -> usize {
    println!("Proceess block {:?}", block);
    let mut grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let r = mirror_on_row(&grid);
    let c = mirror_on_col(&grid);

    let mut answer = 0;
    answer += match r {
        Some(v) => v,
        None => { 0 },
    };

    answer += match c {
        Some(v) => v * 100,
        None => { 0 },
    };


    println!("{:?} {:?}", r, c);
   
    answer
}

fn process_input_lines(haystack: &str) -> usize {
    haystack.split("\n\n").map(|line| process_input_block(line)).sum()
}


fn process_input_line2(line: &str) -> usize {
    0
}

fn process_input_lines2(haystack: &str) -> usize {
   // haystack.lines().map(|line| process_input_line2(line)).sum()
   0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d13_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 13.1: {}", process_input_lines(&contents));
                println!("Puzzle # 13.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
