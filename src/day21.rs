/*
*/

use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::Read;
use std::vec;
use crate::util::util;

fn dfs( n: usize, depth: usize, depth_map:&mut HashMap<usize, HashSet<usize>>, visited: &mut HashSet<usize>, adj_list:&HashMap<usize, HashSet<usize>> ) {

    if depth > 64 { return;}

    visited.insert(n);
    depth_map.entry(depth).or_insert_with(HashSet::new).insert(n);
    
    //println!("Depth {:?} Node {:?} ", depth, n);

    if let Some(paths) = adj_list.get(&n) {
        for p in paths {
            //if !visited.contains(p) {
                dfs(*p, depth+1, depth_map, visited, adj_list);
            //} 
        }
    }     
}

fn bfs(start: usize, steps:usize, adj_list:&HashMap<usize, HashSet<usize>> ) {

    let mut depth_map: HashMap<usize, usize> = HashMap::new();
    let mut visted: HashSet<usize> = HashSet::new();
    let mut q: VecDeque<usize> = VecDeque::new();
    
    depth_map.insert(start,0);
    q.push_back(start);
    visted.insert(start);
    while !q.is_empty() {
        if let Some(t) = q.pop_front() {
            if let Some(children) = adj_list.get(&t) {
                for c in children.iter(){
                    if !visted.contains(c) {
                        let d = depth_map.get(&t).unwrap() + 1;
                        depth_map.insert(*c, d);
                        q.push_back(*c);
                        visted.insert(*c);
                    }
                }
            }
        }
    }

    let mut nodes_by_depth: HashMap<usize,Vec<usize>> = HashMap::new();

    for e in depth_map.iter() {
        nodes_by_depth.entry(*e.1).or_insert_with(Vec::new).push(*e.0); 
    }

    println!("hhh -> {:?} ", depth_map);
    println!("hhh -> {:?} ", nodes_by_depth);
}

fn process_input_block(block: &str) -> usize {
    let grid = block
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    if grid.is_empty() {
        return 0;
    }

    util::print_2d_vec_with_indexes(&grid);
    let mut adj_list:HashMap<usize, HashSet<usize>> = HashMap::new();

    let grid_height = grid.len();
    let grid_width = if grid_height > 0 { grid[0].len() } else { 0 };

    let mut visited: HashSet<usize> = HashSet::new();
    
    let compute_idx = 
        |x:i32, y:i32, grid: &Vec<Vec<char>>| {

            if x < 0 || y < 0 || x as usize >= grid_height || y as usize >= grid_width {
                return Err("Out of range");
            }
            if grid[x as usize][y as usize] == '#'{
                return Err("Cannot go there");
            }

            Ok(x as usize * grid_width + y as usize)
        };

    let mut start = usize::MAX;
    for (i, row) in grid.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            let idx = i as usize * grid_width + j as usize;
            if start == usize::MAX && e == &'S' { start = idx; }
            if let Ok(n_idx) = compute_idx(i as i32 - 1, j as i32, &grid) {
                adj_list.entry(idx).or_insert_with(HashSet::new).insert(n_idx);
            }
            if let Ok(n_idx) = compute_idx(i as i32, j as i32 + 1, &grid) {
                adj_list.entry(idx).or_insert_with(HashSet::new).insert(n_idx);
            }
            if let Ok(n_idx) = compute_idx(i as i32 + 1, j as i32, &grid) {
                adj_list.entry(idx).or_insert_with(HashSet::new).insert(n_idx);
            }
            if let Ok(n_idx) = compute_idx(i as i32, j as i32 - 1, &grid) {
                adj_list.entry(idx).or_insert_with(HashSet::new).insert(n_idx);
            }
        }
    }
    println!("Adjecent List {:?}", adj_list);
    println!("start {:?}", start);
    bfs(start, 7, &adj_list);
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
