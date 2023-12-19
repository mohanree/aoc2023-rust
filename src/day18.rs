/*
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use maplit::hashmap;
use rayon::collections::hash_map;
use regex::Regex;

use crate::util::util;

fn process_input_lines(input: &str) -> i64 {
    let dir_regex = Regex::new(r"^([UDLR]?)\s+(\d+)\s+\(#(\w+)\)$").unwrap();

    let dir_offsets: HashMap<char, (i64, i64)> =
        HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);

    let mut path_points: Vec<(i64, i64)> = vec![(0, 0)];
    let mut boundary_length = 0;

    for line in input.lines() {
        if let Some(caps) = dir_regex.captures(line) {
            let dir = caps
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap_or_default();
            let steps: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            let (offset_x, offset_y) = dir_offsets[&dir];
            let (last_x, last_y) = *path_points.last().unwrap();
            path_points.push((last_x + offset_x * steps, last_y + offset_y * steps));
            boundary_length += steps;
        }
    }

    let polygon_area: i64 = path_points.windows(3).fold(0, |acc, window| {
        acc + window[1].0 * (window[0].1 - window[2].1)
    }) / 2;

    let interior_points = polygon_area.abs() - (boundary_length / 2) + 1;

    interior_points + boundary_length
}

fn process_input_lines2(input: &str) -> i64 {
    let dir_regex = Regex::new(r"^([UDLR]?)\s+(\d+)\s+\(#(\w{5})(\w?)\)$").unwrap();

    let dir_offsets: HashMap<char, (i64, i64)> =
        HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);

    let mut path_points: Vec<(i64, i64)> = vec![(0, 0)];
    let mut boundary_length = 0;

    for line in input.lines() {
        if let Some(caps) = dir_regex.captures(line) {
            let steps = i64::from_str_radix(caps.get(3).unwrap().as_str(), 16)
                .ok()
                .unwrap();
            let dir = match caps.get(4).unwrap().as_str() {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => 'R',
            };

            let (offset_x, offset_y) = dir_offsets[&dir];
            let (last_x, last_y) = *path_points.last().unwrap();
            path_points.push((last_x + offset_x * steps, last_y + offset_y * steps));
            boundary_length += steps;
        }
    }

    let polygon_area: i64 = path_points.windows(3).fold(0, |acc, window| {
        acc + window[1].0 * (window[0].1 - window[2].1)
    }) / 2;

    let interior_points = polygon_area.abs() - (boundary_length / 2) + 1;

    interior_points + boundary_length
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d18_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 18.1: {}", process_input_lines(&contents));
                println!("Puzzle # 18.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
