/*
Ugliest code I ever wrote ... 
*/

use crate::util::util;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::vec;

fn dfs(
    current: (usize, usize),
    end: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    g: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
) -> isize {
    if current == end {
        return 0;
    }

    visited.insert(current);
    let mut max_length = isize::MIN;
    if let Some(paths) = g.get(&current) {
        for (next, length) in paths {
            if !visited.contains(next) {
                max_length = max_length.max(dfs(*next, end, visited, g) + *length as isize);
            }
        }
    }

    visited.remove(&current);
    max_length
}

fn process_input_block(block: &str) -> usize {
    let grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        return 0;
    }

    //util::print_2d_vec_with_indexes(&grid);

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

    let mut g: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();
    let mut loc_s = (usize::MAX, usize::MAX);
    let mut loc_d = (usize::MAX, usize::MAX);
    let mut junctions: Vec<(usize, usize)> = vec![];
    for (i, row) in grid.iter().enumerate() {
        let r = i as isize;
        for (j, e) in row.iter().enumerate() {
            let c = j as isize;
            if *e == '#' {
                continue;
            }
            let mut adj_count = 0;
            for (nr, nc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                if let Some(_) = is_valid(nr, nc) {
                    adj_count += 1;
                    //println!(" C {:?} {:?} -> {:?} " , nr, nc, adj_count);
                }
            }
            if adj_count >= 3 {
                junctions.push((i, j));
                //g.insert((i,j), HashMap::new());
            }
            if grid[i][j] == '.' {
                if loc_s.0 == usize::MAX {
                    loc_s = (i, j);
                }
                loc_d = (i, j);
            }
        }
    }

    junctions.push(loc_s);
    junctions.push(loc_d);
    for j in junctions.iter() {
        g.insert(*j, HashMap::new());
    }

    //println!("Junctions: {:?}", junctions );
    //println!("{:?} -> {:?}", loc_s, loc_d);
    //println!("g: {:?}", g );

    let dirs: HashMap<char, Vec<(isize, isize)>> = [
        ('.', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
        ('>', vec![(0, 1)]),
        ('<', vec![(0, -1)]),
        ('^', vec![(-1, 0)]),
        ('v', vec![(1, 0)]),
    ]
    .into_iter()
    .collect();

    for (src, sink) in &junctions {
        let mut stack = vec![(0, *src, *sink)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((*src, *sink));

        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && junctions.contains(&(r, c)) {
                g.get_mut(&(*src, *sink)).unwrap().insert((r, c), n);
                continue;
            }

            if let Some(directions) = dirs.get(&grid[r][c]) {
                for &(dr, dc) in directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if let Some((r1, c1)) = is_valid(nr, nc) {
                        if !visited.contains(&(r1 as usize, c1 as usize)) {
                            stack.push((n + 1, r1, c1));
                            visited.insert((r1 as usize, c1 as usize));
                        }
                    }
                }
            }
        }
    }
    //println!("g: {:?}", g);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    dfs(loc_s, loc_d, &mut visited, &g) as usize
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

    match File::open("data/d23_input.txt") {
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
