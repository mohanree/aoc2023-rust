/*
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::vec;
use crate::util::util;
use petgraph::Graph;
use petgraph::Undirected;


fn process_input_block(block: &str) -> usize {
    
    let mut g = Graph::<_, _, Undirected>::new_undirected();
    let grid = block
        .lines()
        .for_each(|line| {
            let t1: Vec<&str> = line.split(": ").collect();
            let t2: HashSet<&str> = t1[1].split_whitespace().collect();
            let n1 = g.add_node(t1[0]);
            t2.iter().for_each(|e|{
                let n2 = g.add_node(e);
                g.add_edge(n1, n2, ());
            });
        });
    
    println!("Graph {:?}", g);

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

    match File::open("data/d25_input.txt") {
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
