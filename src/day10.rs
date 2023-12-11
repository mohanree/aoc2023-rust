/*
*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use rayon::iter::ParallelDrainFull;

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

/*
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

*/

fn is_valid(x: isize, y: isize, grid: &Vec<Vec<char>>) -> bool {
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len() && grid[x as usize][y as usize] != '.' 
}

fn bfs_walk(src: (usize, usize), grid: &Vec<Vec<char>>) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();

    q.push_back(src);
    visited.insert(src, None);

    let mut d = 0;
    while let Some((x, y)) = q.pop_front() {
        d += 1;

        let p = grid[x][y];
        for (dx, dy) in PIPE_DIRS.get(&p).unwrap() {
            let new_x = (x as isize) + dx;
            let new_y = (y as isize) + dy;

            let new_loc = (new_x as usize, new_y as usize);

            if is_valid(new_x, new_y, grid) && !visited.contains_key(&new_loc) {
                visited.insert(new_loc, Some((x, y)));
                q.push_back(new_loc);
            }
        }
    }

    //println!("Visted {:?} ", visited);
    let mut paths: Vec<Vec<(usize, usize)>> = vec![];
    let mut max_d = 0;
    for (i,e) in visited.iter().enumerate() {
        let mut c = e.1;
        let mut path : Vec<(usize, usize)> = vec![];
        if c.is_none() { continue; }
        path.push(c.unwrap());
        while true {
            match c {
                Some(t) => { 
                    c = visited.get(t).unwrap();
                    path.push(*t);
                },
                None => break
            }      
        }
        max_d = max_d.max(path.len()-1);
        paths.push(path);
    }

    //println!("Paths {:?} ", paths);
    //println!("MAX {:?} ", max_d);

    max_d
}


fn process_input_lines(haystack: &str) -> usize {
    let grid = haystack
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let s = start_loc(&grid);

    bfs_walk(s, &grid)
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
