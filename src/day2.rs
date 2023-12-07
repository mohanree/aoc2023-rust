/*
--
*/

use regex::Regex;
use std::fs::File;
use std::io::Read;

fn test_cond(t: (i32, i32, i32), cond: (i32, i32, i32)) -> bool {
    t.0 <= cond.0 && t.1 <= cond.1 && t.2 <= cond.2
}

fn process_input_line(line: &str) -> i32 {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let re = Regex::new(r"Game (\d+):(.*)").unwrap();
    let Some(caps) = re.captures(line) else {
        return 0;
    };
    let id: i32 = caps[1].parse::<i32>().unwrap();
    let mut ds: Vec<(i32, i32, i32)> = Vec::new();

    for p in caps[2].split(";") {
        let mut tu = (0, 0, 0);
        let s: Vec<&str> = p.split(",").into_iter().collect();
        for e in s {
            let t: Vec<&str> = e.split(" ").into_iter().collect();
            //println!("{} - {:?}", e, t);
            if t[2] == "red" {
                tu.0 = t[1].parse::<i32>().unwrap();
            } else if t[2] == "green" {
                tu.1 = t[1].parse::<i32>().unwrap();
            } else if t[2] == "blue" {
                tu.2 = t[1].parse::<i32>().unwrap();
            }
        }
        ds.push(tu);
    }

    if ds.iter().all(|x| test_cond(*x, (12, 13, 14))) {
        //println!("id: {} -> {:?}", id, ds);
        //println!("hhh {}" , (2, 12, 15) <= (12, 13, 14));
        return id;
    }
    //println!("id: {} -> {:?}", id, ds);

    0
}

fn process_input(inp: &str) -> i32 {
    inp.lines().map(|x| process_input_line(x)).sum()
}

fn process_input_line2(line: &str) -> i32 {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let re = Regex::new(r"Game (\d+):(.*)").unwrap();
    let Some(caps) = re.captures(line) else {
        return 0;
    };
    let id: i32 = caps[1].parse::<i32>().unwrap();
    let mut ds: Vec<(i32, i32, i32)> = Vec::new();

    for p in caps[2].split(";") {
        let mut tu = (0, 0, 0);
        let s: Vec<&str> = p.split(",").into_iter().collect();
        for e in s {
            let t: Vec<&str> = e.split(" ").into_iter().collect();
            //println!("{} - {:?}", e, t);
            if t[2] == "red" {
                tu.0 = t[1].parse::<i32>().unwrap();
            } else if t[2] == "green" {
                tu.1 = t[1].parse::<i32>().unwrap();
            } else if t[2] == "blue" {
                tu.2 = t[1].parse::<i32>().unwrap();
            }
        }
        ds.push(tu);
    }

    //println!("id: {} -> {:?}", id, ds)
    let (max_r, max_g, max_b) = ds.iter().fold(
        (i32::MIN, i32::MIN, i32::MIN),
        |(max_r, max_g, max_b), &(val1, val2, val3)| (max_r.max(val1), max_g.max(val2), max_b.max(val3)),
    );

    max_r * max_g * max_b
}

fn process_input2(inp: &str) -> i32 {
    inp.lines().map(|x| process_input_line2(x)).sum()
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d2_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 5.1: {}", process_input(&contents));
                println!("Puzzle # 5.2: {}", process_input2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
