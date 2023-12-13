/*
*/

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read; 

fn compute(s: &str, runs: &Vec<usize>, cache: HashMap<(&str, &Vec<usize>), usize>) -> usize {

    println!("Processing  {:?}", s);
    if s.is_empty() {
        if runs.is_empty() { return 1; } else { return 0; }
    } else if runs.is_empty() {
        if s.find('#').is_some() { return 0; } else { return 1; }
    } 

    let ret = 0;
    
    match s.chars().next().unwrap() {
        '.' => {
            compute(s.get(1..).unwrap(), runs, cache);
        },
        '#' => {
            print!("");
        },
        '?' => {

        },
        _ => {

        }

    }

    0
} 

fn generate_combinations(s: &str, runs: &Vec<usize>) -> Vec<String> {
    //println!("{:?}", s);
    let mut queue = VecDeque::new();
    queue.push_back(s.to_string());

    let s: usize = runs.iter().sum();
    let mut r = Vec::new();
    while let Some(current) = queue.pop_front() {
        if let Some(pos) = current.find('?') {
            for c in ['#', '.'] {
                let mut new_str = current.clone();
                new_str.replace_range(pos..pos + 1, &c.to_string());
                queue.push_back(new_str);
            }
        } else {
            if s == current.chars().filter(|e| e == &'#').count() {
                r.push(current);
            }
            
        }
    }

    //println!("size = {:?}" ,r.len());

    r
}


fn process_input_line(line: &str) -> usize {
    let v: Vec<&str> = line.split_whitespace().collect();
    
    let target_run_lengths: Vec<usize> = v[1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    generate_combinations(v[0], &target_run_lengths)
        .iter().filter(|e|{
            let runs: Vec<usize> = e
            .split('.')
            .filter(|x| x.len() > 0)
            .map(|y| y.len())
            .collect();
            target_run_lengths == runs 
        }
    ).count()
}

fn process_input_lines(haystack: &str) -> usize {
    haystack.lines().map(|line| process_input_line(line)).sum()
}


fn process_input_line2(line: &str) -> usize {
    let mut cache: HashMap<&str, usize> = HashMap::new();
    let v: Vec<&str> = line.split_whitespace().collect();
    //println!("{:?}", v[0]);

    let target_run_lengths: Vec<usize> = v[1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let repeated_run_lengths: Vec<_> = (0..5).flat_map(|_| target_run_lengths.iter().cloned()).collect();

    /*
    generate_combinations(&v[0].repeat(5), &repeated_run_lengths)
        .iter().filter(|e|{
            let runs: Vec<usize> = e
            .split('.')
            .filter(|x| x.len() > 0)
            .map(|y| y.len())
            .collect();
            repeated_run_lengths == runs 
        }
    ).count() 
*/
    let cache= HashMap::new();
    compute(&v[0].repeat(5), &repeated_run_lengths, cache)
}

fn process_input_lines2(haystack: &str) -> usize {
    haystack.lines().map(|line| process_input_line2(line)).sum()
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d12_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 12.1: {}", process_input_lines(&contents));
                println!("Puzzle # 12.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
