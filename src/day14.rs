/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn print_2d_vec_with_indexes(grid: &Vec<Vec<char>>) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }

    print!("  ");
    for j in 0..grid[0].len() {
        print!("{:3} ", j);
    }
    println!();

    for (i, row) in grid.iter().enumerate() {
        print!("{:3} ", i);
        for &val in row {
            print!("{:3} ", val);
        }
        println!();
    }
}

fn tilt(grid: &mut Vec<Vec<char>>) -> usize {
    //print_2d_vec_with_indexes(&grid);

    if grid.is_empty() {
        return 0;
    }
    for c in 0..grid.len() {
        let row_len = grid[0].len();
        for i in 0..row_len {
            for j in (i + 1..row_len).rev() {
                if grid[j][c] == 'O' && grid[j - 1][c] == '.' {
                    grid[j][c] = '.';
                    grid[j - 1][c] = 'O';
                    //println!(" Swap ->({:?}, {:?}) -> {:?}", j, c, grid[j][c]);
                } else {
                    //println!(" Skip ->({:?}, {:?}) -> {:?}", j, c, grid[j][c]);
                }
            }
            //println!("");
            //print_2d_vec_with_indexes(grid);
        }
    }

    let mut load_factor = grid.len();
    grid.iter()
        .map(|x| {
            let t = x.iter().filter(|y| *y == &'O').count();
            let r = load_factor * x.iter().filter(|y| *y == &'O').count();
            //println!("{:?} -> {:?} {:?} {:?}", load_factor, r, x, t);
            load_factor -= 1;
            r
        })
        .sum()
}

fn process_input_block(block: &str) -> usize {
    println!("Proceess block {:?}", block);
    let mut grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    tilt(&mut grid)
}

fn process_input_lines(haystack: &str) -> usize {
    haystack
        .split("\n\n")
        .map(|line| process_input_block(line))
        .sum()
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

    match File::open("data/d14_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 14.1: {}", process_input_lines(&contents));
                println!("Puzzle # 14.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
