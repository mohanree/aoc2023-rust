/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn is_valid(x: isize, y: isize, grid: &Vec<Vec<char>>) -> bool {
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
}
// Testing function
fn bfs_walk(src: (usize, usize), dest: (usize, usize), grid: &Vec<Vec<char>>) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();

    q.push_back(src);
    visited.insert(src, None);

    let mut d = 0;

    'outer: while let Some((x, y)) = q.pop_front() {
        d += 1;

        let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for &(dx, dy) in &directions {
            let new_x = (x as isize) + dx;
            let new_y = (y as isize) + dy;

            let new_loc = (new_x as usize, new_y as usize);

            if is_valid(new_x, new_y, grid) && !visited.contains_key(&new_loc) {
                visited.insert(new_loc, Some((x, y)));
                q.push_back(new_loc);
                if new_loc == dest {
                    break 'outer;
                }
            }
        }
    }

    let mut path: Vec<(usize, usize)> = vec![];

    let mut c = dest;
    path.push(c);
    while c != src {
        let t = visited.get(&c).unwrap();
        match t {
            Some(x) => {
                path.push(*x);
                c = *x;
            }
            None => {}
        }
    }

    println!("Path {:?} ", path);

    path.len() - 1
}

fn find_all_galaxies(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, &v)| v == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

fn find_all_galaxies_pairs(grid: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize))> {
    let g: Vec<(usize, usize)> = find_all_galaxies(grid);

    let p: Vec<((usize, usize), (usize, usize))> = g
        .iter()
        .enumerate()
        .flat_map(|(i, &x)| g.iter().skip(i + 1).map(move |&y| (x, y)))
        .collect();
    p
}

fn process_input_lines(haystack: &str) -> usize {
    let mut grid = haystack
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let ex_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_i, r)| r.iter().all(|&x| x == '.'))
        .map(|(i, _r)| i)
        .collect();

    for i in 0..ex_rows.len() {
        grid.insert(ex_rows[i] + i, vec!['.'; grid[0].len()]);
    }

    let ex_cols: Vec<usize> = (0..grid[0].len())
        .filter(|col| (0..grid.len()).all(|row| grid[row][*col] == '.'))
        .collect();

    for (i, e) in ex_cols.iter().enumerate() {
        for row in 0..grid.len() {
            grid[row].insert(e + i, 'T');
        }
    }

    /* println!("Grid");
       grid.iter().for_each(|x| {
           println!("{:?}", x);
       });
        find_all_galaxies_pairs(&grid).iter()
        .map(|(s,d)| bfs_walk(*s, *d, &grid)).sum()
    */

    find_all_galaxies_pairs(&grid)
        .iter()
        .map(|(s, d)| s.0.abs_diff(d.0) + s.1.abs_diff(d.1))
        .sum()
}

fn process_input_lines2(haystack: &str) -> usize {
    let mut grid = haystack
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut all_galaxies = find_all_galaxies(&grid);

    let ex_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_i, r)| r.iter().all(|&x| x == '.'))
        .map(|(i, _r)| i)
        .collect();

    let offset: usize = 1000000 - 1;
    for (i, row) in ex_rows.iter().enumerate() {
        all_galaxies.iter_mut().for_each(|(x, y)| {
            if *x > *row + i * offset {
                *x += offset;
            }
        });
    }

    let ex_cols: Vec<usize> = (0..grid[0].len())
        .filter(|col| (0..grid.len()).all(|row| grid[row][*col] == '.'))
        .collect();

    for (i, col) in ex_cols.iter().enumerate() {
        all_galaxies.iter_mut().for_each(|(x, y)| {
            if *y > *col + i * offset {
                *y += offset;
            }
        });
    }

    let p: Vec<((usize, usize), (usize, usize))> = all_galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &x)| all_galaxies.iter().skip(i + 1).map(move |&y| (x, y)))
        .collect();

    p.iter()
        .map(|(s, d)| s.0.abs_diff(d.0) + s.1.abs_diff(d.1))
        .sum()
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d11_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 11.1: {}", process_input_lines(&contents));
                println!("Puzzle # 11.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
