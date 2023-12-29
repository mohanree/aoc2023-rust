/*
*/

use crate::util::util;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Read;

struct Graph {
    adj_matrix: Vec<Vec<bool>>,
    visited: Vec<bool>,
}

impl Graph {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum DirT {
    North = 1,
    South = 2,
    East = 3,
    West = 4,
    Unknown = 5,
}
impl DirT {
    fn to_str(&self) -> &str {
        match self {
            DirT::North => "^",
            DirT::South => "v",
            DirT::East => ">",
            DirT::West => "<",
            DirT::Unknown => "",
        }
    }
}

fn print_2d_vec_generic(grid: &Vec<Vec<(bool, HashSet<DirT>)>>) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }

    let column_headers = (0..grid[0].len())
        .map(|j| format!("{:^8}", j))
        .collect::<String>();
    println!("      {}", column_headers);

    for (i, row) in grid.iter().enumerate() {
        let row_values = row
            .iter()
            .map(|&(energized, ref directions)| {
                if !energized {
                    ".       ".to_string()
                } else if directions.is_empty() {
                    "?     ".to_string()
                } else {
                    let dir_chars: String = directions.iter().map(DirT::to_str).collect();
                    format!("{:<8}", dir_chars)
                }
            })
            .collect::<String>();
        println!("{:^8} {}", i, row_values);
    }
}

fn get_next(pos: (usize, usize), dir: DirT, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let (row, col) = pos;
    match dir {
        DirT::North if row > 0 => Some((row - 1, col)),
        DirT::South if row + 1 < grid.len() => Some((row + 1, col)),
        DirT::East if col + 1 < grid[0].len() => Some((row, col + 1)),
        DirT::West if col > 0 => Some((row, col - 1)),
        _ => None,
    }
}

fn beam(
    pos: (usize, usize),
    dir: DirT,
    beam_tracker: &mut Vec<Vec<(bool, HashSet<DirT>)>>,
    grid: &Vec<Vec<char>>,
) {
    let (energized, directions) = &mut beam_tracker[pos.0][pos.1];

    if *energized && directions.contains(&dir) {
        return;
    }

    //print!("{:?} - {:?} :", energized, directions);
    *energized = true;
    let t = directions.insert(dir);
    directions.remove(&DirT::Unknown);

    //util::print_2d_vec_with_indexes(&grid);
    //println!("{:?} {:?} - {:?} - {:?} x {:?}", pos, dir.to_str(), energized, directions, t);
    //print_2d_vec_generic(&beam_tracker);

    let next_dirs = match (grid[pos.0][pos.1], dir) {
        ('|', DirT::North) | ('|', DirT::South) => vec![dir],
        ('|', _) => vec![DirT::North, DirT::South],
        ('-', DirT::East) | ('-', DirT::West) => vec![dir],
        ('-', _) => vec![DirT::East, DirT::West],
        ('\\', DirT::South) => vec![DirT::East],
        ('\\', DirT::North) => vec![DirT::West],
        ('\\', DirT::East) => vec![DirT::South],
        ('\\', DirT::West) => vec![DirT::North],
        ('/', DirT::North) => vec![DirT::East],
        ('/', DirT::South) => vec![DirT::West],
        ('/', DirT::West) => vec![DirT::South],
        ('/', DirT::East) => vec![DirT::North],
        _ => vec![dir],
    };

    for next_dir in next_dirs {
        if let Some(next_pos) = get_next(pos, next_dir, grid) {
            beam(next_pos, next_dir, beam_tracker, grid);
        }
    }
}

fn process_input_block(block: &str) -> usize {
    //println!("Proceess block {:?}", block);
    let grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        return 0;
    }

    //util::print_2d_vec_with_indexes(&grid);
    let mut beam_tracker: Vec<Vec<(bool, HashSet<DirT>)>> =
        vec![vec![(false, HashSet::from([DirT::Unknown])); grid[0].len()]; grid.len()];
    beam((0, 0), DirT::East, &mut beam_tracker, &grid);

    //print_2d_vec_generic(&beam_tracker);

    beam_tracker
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&(energized, _)| energized)
        .count()
}

fn process_input_lines(haystack: &str) -> usize {
    process_input_block(haystack)
}

fn process_input_block2(block: &str) -> usize {
    //println!("Proceess block {:?}", block);
    let grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        return 0;
    }

    let grid_height = grid.len();
    let grid_width = if grid_height > 0 { grid[0].len() } else { 0 };

    //util::print_2d_vec_with_indexes(&grid);

    let mut starts: Vec<(usize, usize, DirT)> = vec![];

    for i in 0..grid_width {
        if grid[0][i] == '.' {
            starts.push((0, i, DirT::South));
        }
        if grid[grid_height - 1][i] == '.' {
            starts.push((grid_height - 1, i, DirT::North));
        }
    }

    for i in 0..grid_height {
        if grid[i][0] == '.' {
            starts.push((i, 0, DirT::East))
        }
        if grid[i][grid_width - 1] == '.' {
            starts.push((i, grid_width - 1, DirT::West))
        }
    }

    //println!("Starts {:?}", starts);

    let beam_me_up_scotty = |start: (usize, usize), dir: DirT| -> usize {
        let mut beam_tracker: Vec<Vec<(bool, HashSet<DirT>)>> =
            vec![vec![(false, HashSet::from([DirT::Unknown])); grid[0].len()]; grid.len()];
        beam(start, dir, &mut beam_tracker, &grid);

        //print_2d_vec_generic(&beam_tracker);

        beam_tracker
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&(energized, _)| energized)
            .count()
    };

    let mut max_beam = 0;
    starts.iter().for_each(|e| {
        max_beam = beam_me_up_scotty((e.0, e.1), e.2).max(max_beam);
    });

    max_beam
}

fn process_input_lines2(haystack: &str) -> usize {
    process_input_block2(haystack)
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d16_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 16.1: {}", process_input_lines(&contents));
                println!("Puzzle # 16.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
