/*
*/

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::f64;
use std::fs::File;
use std::i64;
use std::io::Read;

fn process_input_lines(input: &str) -> i64 {
    let dir_regex =
        Regex::new(r"([-\d]+),\s*([-\d]+),\s*([-\d]+)\s*@\s*([-\d]+),\s*([-\d]+),\s*([-\d]+)")
            .unwrap();

    let mut hailstones: Vec<((f64, f64, f64), (f64, f64, f64))> = vec![];

    let test_area = (200000000000000.0, 400000000000000.0); //(7.0,27.0);

    for line in input.lines() {
        if let Some(caps) = dir_regex.captures(line) {
            let parse_or_zero = |i: usize| {
                caps.get(i)
                    .and_then(|m| m.as_str().parse::<f64>().ok())
                    .unwrap_or(0.0)
            };

            let p = (parse_or_zero(1), parse_or_zero(2), parse_or_zero(3));
            let d = (parse_or_zero(4), parse_or_zero(5), parse_or_zero(6));

            hailstones.push((p, d));
        }
    }

    let mut ret = 0;

    for (idx, hs1) in hailstones.iter().enumerate() {
        let m1 = hs1.1 .1 / hs1.1 .0;
        let (x1, y1, z1) = (hs1.0 .0, hs1.0 .1, hs1.0 .2);
        for hs2 in hailstones.iter().skip(idx + 1) {
            if hs1 == hs2 {
                continue;
            }
            let (x2, y2, z2) = (hs2.0 .0, hs2.0 .1, hs2.0 .2);

            let m2 = hs2.1 .1 / hs2.1 .0;

            if m1 == m2 {
                continue;
            }

            let x = (y2 - y1 + m1 * x1 - m2 * x2) / (m1 - m2);
            let y = m1 * x + (y1 - m1 * x1);

            if ((x >= test_area.0 && x <= test_area.1) && (y >= test_area.0 && y <= test_area.1)) {
                let t1x = (x - hs1.0 .0) * hs1.1 .0;
                let t1y = (y - hs1.0 .1) * hs1.1 .1;
                let t2x = (x - hs2.0 .0) * hs2.1 .0;
                let t2y = (y - hs2.0 .1) * hs2.1 .1;

                if (t1x >= 0.0 && t1y >= 0.0) && (t2x >= 0.0 && t2y >= 0.0) {
                    ret += 1;
                }
            }
        }
    }
    ret
}

fn process_input_lines2(input: &str) -> i64 {
    0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d24_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 24.1: {}", process_input_lines(&contents));
                // println!("Puzzle # 18.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
