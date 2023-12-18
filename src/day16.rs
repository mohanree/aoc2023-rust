/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;


struct Graph {
    adj_matrix: Vec<Vec<bool>>,
    visited: Vec<bool>,
    grid: &Vec<Vec<char>>,
}

impl Graph {
    fn new( grid: &Vec<Vec<char>> ) {

    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
enum DirT {
    North = 1,
    South = 2,
    East = 3,
    West = 4,
}

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

fn print_2d_vec_with_indexes_b(grid: &Vec<Vec<bool>>) {
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

fn is_valid(pos: (usize, usize), grid: &Vec<Vec<char>>) -> bool {

    if pos.0 < 0 || pos.1 < 0 {
        return false;
    }
    if pos.0 >= grid.len() || pos.1 >= grid[0].len() {
        return false;
    }

    true
}

fn get_next( pos: (usize, usize), dir: DirT , grid: &Vec<Vec<char>>) -> Option<(usize, usize)>{

    match dir {
        DirT::North =>  {
            if pos.0 != 0 {
                return Some((pos.0 - 1, pos.1));
            }
        },
        DirT::South =>  {
            if pos.0 < grid.len() - 1   {
                return Some((pos.0+1, pos.1));
            }
        },
        DirT::East =>  {
            if pos.1 < grid[0].len() - 1   {
                return Some((pos.0, pos.1+1));
            }
        },
        DirT::West =>  {
            if pos.1 != 0 {
                return Some((pos.0, pos.1-1));
            }
        },
    }

    None
}

fn beam(pos: (usize, usize), dir: DirT, visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>) {
    //print_2d_vec_with_indexes(&grid);
    //print_2d_vec_with_indexes_b(&visited);
    print!("{:?} {:?}, ", pos, dir);

    visited[pos.0][pos.1] = true;
   

    match grid[pos.0][pos.1] {
        '|' => {
            if dir == DirT::North || dir == DirT::South {
                if let Some(t) = get_next(pos, dir, grid) {
                    beam(t, dir, visited, grid);
                }
            }
            else {
                if let Some(t) = get_next(pos, DirT::North, grid) {
                    beam(t, DirT::North, visited, grid);
                }
                if let Some(t) = get_next(pos, DirT::South, grid) {
                    beam(t, DirT::South, visited, grid);
                }
            }

        },
        '-' => {
            if dir == DirT::West || dir == DirT::East {
                if let Some(t) = get_next(pos, dir, grid) {
                    beam(t, dir, visited, grid);
                }
            }
            else {
                if let Some(t) = get_next(pos, DirT::East, grid) {
                    beam(t, DirT::East, visited, grid);
                }
                if let Some(t) = get_next(pos, DirT::West, grid) {
                    beam(t, DirT::West, visited, grid);
                }
            }

        },
        '\\' => {
            if dir == DirT::South {
                if let Some(t) = get_next(pos, DirT::East, grid) {
                    beam(t, DirT::East, visited, grid);
                }
            }
            else if dir == DirT::North {
                if let Some(t) = get_next(pos, DirT::West, grid) {
                    beam(t, DirT::West, visited, grid);
                }
            }
            else if dir == DirT::East {
                if let Some(t) = get_next(pos, DirT::South, grid) {
                    beam(t, DirT::South, visited, grid);
                }
            }
            else if dir == DirT::West {
                if let Some(t) = get_next(pos, DirT::North, grid) {
                    beam(t, DirT::North, visited, grid);
                }
            }

        },
        '/' => {
            if dir == DirT::North {
                if let Some(t) = get_next(pos, DirT::East, grid) {
                    beam(t, DirT::East, visited, grid);
                }
            }
            else if dir == DirT::South {
                if let Some(t) = get_next(pos, DirT::West, grid) {
                    beam(t, DirT::West, visited, grid);
                }
            }
            else if dir == DirT::West {
                if let Some(t) = get_next(pos, DirT::South, grid) {
                    beam(t, DirT::South, visited, grid);
                }
            }
            else if dir == DirT::East {
                if let Some(t) = get_next(pos, DirT::North, grid) {
                    beam(t, DirT::North, visited, grid);
                }
            }
        },
        _ => {
            if let Some(t) = get_next(pos, dir, grid) {
                beam(t, dir, visited, grid);
            }
        }
    }
    
}

fn process_input_block(block: &str) -> usize {
    println!("Proceess block {:?}", block);
    let grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        return 0;
    }

    print_2d_vec_with_indexes(&grid);
    let mut visited: Vec<Vec<bool>> = vec![vec![false;grid[0].len()];grid.len()];
    beam((0,0), DirT::East, &mut visited, &grid);

    print_2d_vec_with_indexes_b(&visited);

    0
}

fn process_input_lines(haystack: &str) -> usize {
    process_input_block(haystack)
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

    match File::open("data/d16_input.txt") {
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
