/*
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn start_loc(grid: &Vec<Vec<char>>) -> (i64, i64) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                return (i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
    }

    (0, 0)
}

fn is_valid(loc: (i64, i64), grid: &Vec<Vec<char>>) -> bool {
    if loc.0 < 0 || loc.0 >= grid.len().try_into().unwrap() {
        return false;
    }
    if loc.1 < 0 || loc.1 >= grid[0].len().try_into().unwrap() {
        return false;
    }
    if grid[loc.0 as usize][loc.1 as usize] == '.' {
        return false;
    }

    true
}

fn dfs(
    loc: (i64, i64),
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i64, i64)>,
    dist: i64,
) -> i64 {
    if !is_valid(loc, grid) || visited.contains(&loc) {
        return dist;
    }

    visited.insert(loc);
    println!("Loc -> {:?} {:?} {:?} ", loc, dist, visited);

    let mut max_dist = dist;
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (dx, dy) in dirs {
        let n_loc = (loc.0 + dx, loc.1 + dy);
        if is_valid(n_loc, grid)  {
            max_dist = max_dist.max(dfs(n_loc, grid, visited, dist + 1))
        }
    }

    max_dist
}

fn process_input_lines(haystack: &str) -> i64 {
    let grid = haystack
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let s = start_loc(&grid);

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let max_dist = dfs(s, &grid, &mut visited, 0);

    println!("Grid {:?} -> {:?}", grid, s);
    max_dist
}

fn process_input_lines2(haystack: &str) -> u32 {
    0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d10_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 10.1: {}", process_input_lines(&contents));
                println!("Puzzle # 10.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
