/*
*/


use std::fs::File;
use std::io::{Read};

fn process_input_line(line: &str) -> i32 {
    let d: Vec<i32> = line.split_whitespace()
    .map(|s| s.parse::<i32>().expect("Failed to parse as i32"))
    .collect();

    let mut haystack: Vec<Vec<i32>> = vec![d.clone()];
    let mut t_v: &Vec<i32> = &haystack.last().unwrap();
    while  t_v.iter().any(|x| *x != 0) && t_v.len() > 1 {
        let n = t_v.windows(2).map(|window| window[1]-window[0]).collect();
        haystack.push(n);
         t_v = &haystack.last().unwrap();
    }

    let mut e : i32 = 0; 
    for v in haystack.iter_mut().rev() {
        e = v.last().unwrap() + e;
        v.push(e);
    }

    //println!("New hay {:?}", haystack);

    *haystack[0].last().unwrap()
}

fn process_input_lines(haystack: &str) -> i32 {
    haystack.lines().map(|line| process_input_line(line)).sum()
}


fn process_input_lines2(haystack: &str) -> i32 {
    haystack.lines().map(|line| process_input_line2(line)).sum()
}

fn process_input_line2(line: &str) -> i32 {   
    let d: Vec<i32> = line.split_whitespace()
    .map(|s| s.parse::<i32>().expect("Failed to parse as i32"))
    .collect();

    let mut haystack: Vec<Vec<i32>> = vec![d.clone()];
    let mut t_v: &Vec<i32> = &haystack.last().unwrap();
    while  t_v.iter().any(|x| *x != 0) && t_v.len() > 1 {
        let n = t_v.windows(2).map(|window| window[1]-window[0]).collect();
        haystack.push(n);
         t_v = &haystack.last().unwrap();
    }

    let mut e : i32 = 0; 
    for v in haystack.iter_mut().rev() {
        e = v.first().unwrap() - e ;
        v.insert(0, e);
    }

    println!("New hay {:?}", haystack);

    *haystack[0].first().unwrap()
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d9_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 9.1: {}", process_input_lines(&contents));
                println!("Puzzle # 9.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
