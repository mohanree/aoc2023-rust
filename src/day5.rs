
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;


fn find_custom_range_lookup( range_map: &Vec<(u64, u64, u64)>, key: u64) -> u64 {
    let mut ret = key;
    for e in range_map {
        let r = e.0..(e.0+e.2);
        
        if r.contains(&key) {
            ret = key-e.0+e.1;
            return ret;
        }   
    }
    ret
}

fn extract_seeds_case_1(line: &str) -> Vec<u64> {
    line
    .split(':')
    .nth(1)
    .unwrap()
    .split_whitespace()
    .map(|num| num.parse().unwrap())
    .collect()
}

fn extract_seeds_case_2(line: &str) -> HashSet<u64> {
    let mut s = extract_seeds_case_1(line);

    let mut seeds : HashSet<u64> = HashSet::new();

    for chunk in s.chunks(2) {
        if let [s, r] = chunk {
            println!("{} {}", s, r);
            for e in  (*s)..(*s + *r) {
                seeds.insert(e);
            }
        }
    }
    seeds
}

fn process_input_mod(inp: &str) -> u64 {
    let sections: Vec<&str> = inp.split("\n\n").collect();

    let mut seed2soil: Vec<(u64, u64, u64)> = vec![];
    let mut soil2fertilizer: Vec<(u64, u64, u64)> = vec![];
    let mut fertilizer2water: Vec<(u64, u64, u64)> = vec![];
    let mut water2light: Vec<(u64, u64, u64)> = vec![];
    let mut light2temperature: Vec<(u64, u64, u64)> = vec![];
    let mut temperature2humidity: Vec<(u64, u64, u64)> = vec![];
    let mut humidity2location: Vec<(u64, u64, u64)> = vec![];

    for &section in &sections[1..] {
        if section.is_empty() {
            continue;
        }

        let mut m : Vec<(u64, u64, u64)> = vec![];
        let mut lines = section.lines();
        let header = lines.next().unwrap();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let v: Vec<u64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            m.push((v[1], v[0], v[2]));
        }
        match header {
            "seed-to-soil map:" => {
                seed2soil.clone_from(&m);
            }
            "soil-to-fertilizer map:" => {
                soil2fertilizer.clone_from(&m);
            }
            "fertilizer-to-water map:" => {
                fertilizer2water.clone_from(&m);
            }
            "water-to-light map:" => {
                water2light.clone_from(&m);
            }
            "light-to-temperature map:" => {
                light2temperature.clone_from(&m);
            }
            "temperature-to-humidity map:" => {
                temperature2humidity.clone_from(&m);
            }
            "humidity-to-location map:" => {
                humidity2location.clone_from(&m);
            }
            _ => {}
        }
    }

    let mut m = u64::MAX;

    let mut s = extract_seeds_case_1(sections[0]);
    for chunk in s.chunks(2) {
        if let [s, r] = chunk {
            println!("\n{:20} {:20}", s, r);
            let mut i : u64 = 0;
            for k in  (*s)..(*s + *r) {
                
                let l :u64;
                let l1 = find_custom_range_lookup(&seed2soil, k);
                let l2 = find_custom_range_lookup(&soil2fertilizer, l1);
                let l3 = find_custom_range_lookup(&fertilizer2water, l2);
                let l4 = find_custom_range_lookup(&water2light, l3);
                let l5 = find_custom_range_lookup(&light2temperature, l4);
                let l6 = find_custom_range_lookup(&temperature2humidity, l5);
                let l7 = find_custom_range_lookup(&humidity2location, l6);

                l = l7;            
                if l < m {
                    m = l;
                }
             /*    println!(
                    "Chaining [ {} => {} => {} => {} => {} => {} => {} => {} ]",
                    k, l1, l2, l3, l4, l5, l6, l7
                ); */

                if i % 10000 == 0{
                    print!("\r{:20} {:20} ", "Counting: ", i);
                }
                i = i+1;
            }
            println!("\n{:20} {:20} {:20}", s, r, m);
        }
    }

    m
}



fn process_input(inp: &str, ranged_seeds: bool) -> u64 {
    let sections: Vec<&str> = inp.split("\n\n").collect();

    let seeds : HashSet<u64>;
    if ranged_seeds {
        seeds = extract_seeds_case_2(sections[0]);
    }
    else {
        let seeds_all = extract_seeds_case_1(sections[0]);
        seeds = seeds_all.into_iter().collect();
    }

    println!("Number of seeds to process = {}", seeds.len());

    let mut seed2soil: Vec<(u64, u64, u64)> = vec![];
    let mut soil2fertilizer: Vec<(u64, u64, u64)> = vec![];
    let mut fertilizer2water: Vec<(u64, u64, u64)> = vec![];
    let mut water2light: Vec<(u64, u64, u64)> = vec![];
    let mut light2temperature: Vec<(u64, u64, u64)> = vec![];
    let mut temperature2humidity: Vec<(u64, u64, u64)> = vec![];
    let mut humidity2location: Vec<(u64, u64, u64)> = vec![];

    for &section in &sections[1..] {
        if section.is_empty() {
            continue;
        }

        let mut m : Vec<(u64, u64, u64)> = vec![];
        let mut lines = section.lines();
        let header = lines.next().unwrap();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let v: Vec<u64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            m.push((v[1], v[0], v[2]));
        }
        match header {
            "seed-to-soil map:" => {
                seed2soil.clone_from(&m);
            }
            "soil-to-fertilizer map:" => {
                soil2fertilizer.clone_from(&m);
            }
            "fertilizer-to-water map:" => {
                fertilizer2water.clone_from(&m);
            }
            "water-to-light map:" => {
                water2light.clone_from(&m);
            }
            "light-to-temperature map:" => {
                light2temperature.clone_from(&m);
            }
            "temperature-to-humidity map:" => {
                temperature2humidity.clone_from(&m);
            }
            "humidity-to-location map:" => {
                humidity2location.clone_from(&m);
            }
            _ => {}
        }
    }

    let mut m = u64::MAX;
    let mut perc : u64 = 0;
    let mut i : u64 = 0;
    let len = seeds.len() as u64;
    for k in seeds {
        let l1 = find_custom_range_lookup(&seed2soil, k);
        let l2 = find_custom_range_lookup(&soil2fertilizer, l1);
        let l3 = find_custom_range_lookup(&fertilizer2water, l2);
        let l4 = find_custom_range_lookup(&water2light, l3);
        let l5 = find_custom_range_lookup(&light2temperature, l4);
        let l6 = find_custom_range_lookup(&temperature2humidity, l5);
        let l7 = find_custom_range_lookup(&humidity2location, l6);

        if l7 < m {
            m = l7;
        }
        /*    println!(
            "Chaining [ {} => {} => {} => {} => {} => {} => {} => {} ]",
            k, l1, l2, l3, l4, l5, l6, l7
        ); */
        
        let new_perc = i/len * 100;
        if new_perc - perc > 10 {
            perc = new_perc;
            println!("% lookup done is {}", perc );
        }
        i = i+1;
    } 
    m
}

pub fn lowest_location_number() {
    let mut contents = String::new();

    match File::open("data/d5_input.txt") {
        Ok(mut file) => {
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("Puzzle # 5.1: {}", process_input(&contents,false));
                    println!("Puzzle # 5.2: {}", process_input_mod(&contents));
                }
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}
