/*
*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

lazy_static! {
    static ref PIPE_DIRS: HashMap<char, Vec<(isize, isize)>> = {
        let mut m = HashMap::new();
        m.insert('|', vec![(-1, 0), (1, 0)]);
        m.insert('-', vec![(0, -1), (0, 1)]);
        m.insert('L', vec![(-1, 0), (0, 1)]);
        m.insert('J', vec![(-1, 0), (0, -1)]);
        m.insert('7', vec![(0, -1), (1, 0)]);
        m.insert('F', vec![(1, 0), (0, 1)]);
        m.insert('.', vec![]);
        m.insert('S', vec![(-1, 0), (0, -1), (1, 0), (0, 1)]);
        m
    };
}

fn start_loc(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn is_valid(loc: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
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

fn bfs_walk ( loc: (usize, usize), grid: &Vec<Vec<char>>) {

    let mut q: VecDeque <(usize, usize)> = VecDeque::new();
    let mut visted = vec!(vec!(false;grid[0].len());grid.len());
    q.push_back(loc);
    visted[loc.0][loc.1] = true;

    while let Some(t) = q.pop_front() {
        let p = grid[t.0][t.1];
        println!("{:?} {:?}", t, q);
        for (dx, dy) in PIPE_DIRS.get(&p).unwrap() {
            let new_loc = (t.0 as isize + dx, t.1 as isize + dy);

            if is_valid(new_loc, grid) && !visted[new_loc.0 as usize][new_loc.1 as usize] {
                visted[new_loc.0 as usize][new_loc.0 as usize] = true;
                q.push_back((new_loc.0 as usize, new_loc.0 as usize));
            }
        }
    }
}

fn process_input_lines(haystack: &str) -> isize {
    let grid = haystack
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let s = start_loc(&grid);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut node2dist: HashMap<(isize, isize), isize> = HashMap::new();
    bfs_walk(s, &grid);

    println!("Grid {:?} -> {:?}", grid, s);
    let mut u_grid: Vec<Vec<isize>>  = vec![vec![-1;grid[0].len()];grid.len()];
    for (e, v) in node2dist.iter() {
    //    u_grid[e.0 as isize][e.1 as isize] = *v;
    }

    println!("Node2Dist {:?}", node2dist);
    println!("Ugrid {:?}", u_grid);

    0
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
