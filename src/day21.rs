/*
*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

use crate::util::util;

fn process_input_block(block: &str) -> usize {
    let grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        return 0;
    }

    //util::print_2d_vec_with_indexes(&grid);
    let mut adj_list: HashMap<usize, HashSet<usize>> = HashMap::new();

    let grid_height = grid.len();
    let grid_width = if grid_height > 0 { grid[0].len() } else { 0 };

    let is_valid = |x: isize, y: isize| {
        if x < 0 || y < 0 || x as usize >= grid_height || y as usize >= grid_width {
            return None;
        }

        if grid[x as usize][y as usize] == '#' {
            return None;
        }

        Some((x as usize, y as usize))
    };

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| *c == &'S')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    //println!("start {:?}", start);

    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut sol: HashSet<(usize, usize)> = HashSet::new();

    q.push_back((start.0, start.1, 64));
    visited.insert(start);
    while let Some((r, c, d)) = q.pop_front() {
        let (x, y) = (r as isize, c as isize);
        if d % 2 == 0 { // hacky for now
            sol.insert((r, c));
        }
        if d == 0 {
            continue;
        }
        for (nx, ny) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            if let Some((nr, nc)) = is_valid(nx, ny) {
                if visited.contains(&(nr, nc)) {
                    continue;
                }
                visited.insert((nr, nc));
                q.push_back((nr, nc, d - 1))
            }
        }
    }

    //println!("Solution {:?}", sol);

    sol.len()
}

fn process_input_lines(haystack: &str) -> usize {
    process_input_block(haystack)
}

fn process_input_line2(_line: &str) -> usize {
    0
}

fn process_input_lines2(_haystack: &str) -> usize {
    // haystack.lines().map(|line| process_input_line2(line)).sum()
    0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d21_input.txt") {
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
