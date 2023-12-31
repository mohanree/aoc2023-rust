/*
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use petgraph::visit::EdgeRef;
use regex::Regex;
use evalexpr::*;

use crate::util::util;

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn parse_to_struct(input: &str) -> Option<Part> {
    let mut values = HashMap::new();
    let pairs = input.trim_matches(|c| c == '{' || c == '}').split(',');

    for pair in pairs {
        let mut parts = pair.trim().split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            values.insert(key, value.parse::<i32>().ok()?);
        }
    }

    Some(Part {
        x: *values.get("x")?,
        m: *values.get("m")?,
        a: *values.get("a")?,
        s: *values.get("s")?,
    })
}

fn run_workflow( part: &Part, wf: &HashMap<&str, Vec<(&str, &str)>> ) -> i32 {

    let mut r = "in";
    
    println!("");
    'outer: while let Some(sub_rules) = wf.get(r) {
        print!("{:?}->", r);
        for (e, t) in sub_rules {
            //print!("{:?}->", r);

            let mut c = HashMapContext::new();

            c.set_value("a".into(), Value::from(part.a as i64)).unwrap();
            c.set_value("m".into(), Value::from(part.m as i64)).unwrap();
            c.set_value("s".into(), Value::from(part.s as i64)).unwrap();
            c.set_value("x".into(), Value::from(part.x as i64)).unwrap();

            if let Ok(result) = eval_with_context(e, &c) {
                //println!("{:?}", result);
                if result == Value::from(true) {
                    r = t;
                    if t == &"R" || t == &"A" {
                        print!("{:?}", t);
                        break;
                    }
                    else {
                        continue 'outer; 
                    }
                }    
            }
        }
        break;
    }
    println!("");

    if r == "A" {
        return part.a + part.m + part.s + part.x ;
    }
    0
}

fn process_input_lines(input: &str) -> i32 {
    let line_regex = Regex::new(r"^(\w+)\{([^\{\}]+)\}$").unwrap();
    let mut wf: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    let mut parts: Vec<Part> = vec![];

    for line in input.lines() {
        if let Some(caps) = line_regex.captures(line) {
            let rule_name = caps.get(1).unwrap().as_str();
            let rules_str = caps.get(2).unwrap().as_str();
            let rules: Vec<(&str, &str)> = rules_str.split(',')
                .map(|rule| {
                    let mut parts = rule.split(':');
                    let t1 =  parts.next().unwrap_or("");
                    let t2 = parts.next().unwrap_or("");
                    if rule.contains(':') { 
                        (t1, t2) 
                    } else {
                        ("true", t1)
                    }
                })
                .collect();
            wf.insert(rule_name, rules);
        } else {
            if let Some(part) = parse_to_struct(line) {
                parts.push(part);
            }
        }

    }

    println!("Workflow rules: {:?}", wf);
    println!("Parts: {:?}", parts);

    parts.iter().map(|e|{
        run_workflow(&e, &wf)
    }).sum()
    
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

    match File::open("data/d19_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 19.1: {}", process_input_lines(&contents));
                //println!("Puzzle # 18.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
