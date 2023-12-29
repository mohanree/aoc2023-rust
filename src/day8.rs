/*
*/

use std::collections::{HashMap};
use std::fs::File;
use std::io::Read;

use regex::Regex;

fn process_input_lines(haystack: &str) -> u32 {
    let mut lines = haystack.lines();
    let inst = lines.next().unwrap_or_default();
    let re = Regex::new(r"^(\w+)\s*=\s*\((\w+),\s*(\w+)\)$").unwrap();
    let mut data: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        if let Some(caps) = re.captures(line) {
            data.insert(
                caps.get(1).unwrap().as_str(),
                (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
            );
        }
    }

    let mut cursor = "AAA";
    let mut counter = 0;

    'hop: loop {
        for c in inst.chars() {
            if let Some(&(left, right)) = data.get(cursor) {
                cursor = if c == 'R' { right } else { left };
                counter += 1;
                if cursor == "ZZZ" {
                    break 'hop;
                }
            } else {
                break 'hop;
            }
        }
    }

    counter
}

fn process_input_lines2(haystack: &str) -> u32 {
    let mut lines = haystack.lines();
    let inst = lines.next().unwrap_or_default();
    let re = Regex::new(r"^(\w+)\s*=\s*\((\w+),\s*(\w+)\)$").unwrap();
    let mut data: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut source_vec: Vec<&str> = vec![];
    //let mut visited: HashSet<&str> = HashSet::new();
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let t = caps.get(1).unwrap().as_str();
            if t.ends_with('A') {
                source_vec.push(t);
            }
            data.insert(
                t,
                (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
            );
        }
    }

    let mut counter = 0;

    println!("Source vec: {:?}", source_vec);

    let mut next_source_vec: Vec<&str> = Vec::with_capacity(source_vec.len());
    'hop: loop {
        for c in inst.chars() {
            counter += 1;
            let mut all_end_with_z = true;
            next_source_vec.clear();
            //println!("Source vec: {:?}", source_vec);
            for &s in &source_vec {
                if let Some(&(left, right)) = data.get(s) {
                    let next_cursor = if c == 'R' { right } else { left };
                    next_source_vec.push(next_cursor);
                    if !next_cursor.ends_with('Z') {
                        all_end_with_z = false;
                    }
                } else {
                    break 'hop;
                }
            }
            if all_end_with_z {
                break 'hop;
            } 
            
            //source_vec = next_source_vec;
            std::mem::swap(&mut source_vec, &mut next_source_vec);            

            if counter % 100000 == 0 {
                print!("\r{:20} {:20} ", "Counting: ", counter);
            } 
        }
    }
    counter
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d8_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 5.1: {}", process_input_lines(&contents));
                println!("Puzzle # 5.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
