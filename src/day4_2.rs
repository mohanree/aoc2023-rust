use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn no_of_matches_by_card_minus_one(haystack: &str) -> Vec<usize> {
    haystack
        .lines()
        .map(|line| no_of_matches(line))
        .collect::<Vec<_>>()
}

fn no_of_matches(haystack: &str) -> usize {
    let parts: Vec<&str> = haystack[haystack.find(":").unwrap()..].split('|').collect();
    let my_nums: HashSet<i32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let win_nums: HashSet<i32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let i: HashSet<_> = my_nums.intersection(&win_nums).cloned().collect();

    i.len()
}

pub fn no_of_scratch_cards() {
    let mut contents = String::new();

    match File::open("data/d4_input.txt") {
        Ok(mut file) => {
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    //println!("Puzzle # 4.2: {}", no_of_matches_by_card_minus_one(&contents))
                    let indexed_matches = no_of_matches_by_card_minus_one(&contents);
                    let _indexed_card_count: Vec<usize> = vec![0, indexed_matches.len()];
                }
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}
