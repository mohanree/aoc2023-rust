/*
*/

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn calculate_gear_ratio(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    regex: &Regex,
) -> u32 {
    if grid[row][col] != '*' {
        return 0;
    }

    let mut adjacent_numbers = Vec::new();

    for r in row.saturating_sub(1)..=usize::min(row + 1, grid.len() - 1) {
        for c in col.saturating_sub(1)..=usize::min(col + 1, grid[0].len() - 1) {
            if r != row || c != col {
                let line_str = grid[r].iter().collect::<String>();
                regex.find_iter(&line_str).for_each(|m| {
                    if m.start() <= c && m.end() > c {
                        if let Ok(num) = m.as_str().parse::<u32>() {
                            adjacent_numbers.push(num);
                        }
                    }
                });
            }
        }
    }

    adjacent_numbers.sort_unstable();
    adjacent_numbers.dedup();

    if adjacent_numbers.len() == 2 {
        adjacent_numbers[0] * adjacent_numbers[1]
    } else {
        0
    }
}

fn process_input_lines_for_gears(engine_schematic: &str) -> u32 {
    let regex = Regex::new(r"\d+").unwrap();
    let grid: Vec<Vec<char>> = engine_schematic
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut gear_ratio_sum = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            gear_ratio_sum += calculate_gear_ratio(&grid, row, col, &regex);
        }
    }
    gear_ratio_sum
}

fn is_adjacent_to_symbol(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    symbols: &HashSet<char>,
    adjacency_cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if let Some(&cached) = adjacency_cache.get(&(row, col)) {
        return cached;
    }

    let row_start = if row > 0 { row - 1 } else { 0 };
    let row_end = if row < grid.len() - 1 { row + 1 } else { row };
    let col_start = if col > 0 { col - 1 } else { 0 };
    let col_end = if col < grid[0].len() - 1 {
        col + 1
    } else {
        col
    };

    let adjacent = (row_start..=row_end).any(|r| {
        (col_start..=col_end).any(|c| (r != row || c != col) && symbols.contains(&grid[r][c]))
    });

    adjacency_cache.insert((row, col), adjacent);
    adjacent
}

fn process_input_lines(engine_schematic: &str) -> u32 {
    let regex = Regex::new(r"\d+").unwrap();
    let symbols = HashSet::from([
        '*', '#', '+', '$', '!', '@', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']',
        '{', '}', '|', '\\', ';', ':', '\'', ',', '/', '<', '>', '?',
    ]);

    let grid: Vec<Vec<char>> = engine_schematic
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut adjacency_cache = HashMap::new();
    let mut part_number_sum = 0;

    for (row, line) in engine_schematic.lines().enumerate() {
        let mut checked_indices = HashSet::new();
        for m in regex.find_iter(line) {
            let part_number: u32 = m.as_str().parse().unwrap();
            let col = m.start();

            if !checked_indices.contains(&col)
                && (0..m.as_str().len()).any(|offset| {
                    is_adjacent_to_symbol(&grid, row, col + offset, &symbols, &mut adjacency_cache)
                })
            {
                part_number_sum += part_number;
                checked_indices.insert(col);
            }
        }
    }
    part_number_sum
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d3_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 3.1: {}", process_input_lines(&contents));
                println!("Puzzle # 3.2: {}", process_input_lines_for_gears(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
