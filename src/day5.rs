
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;

fn print_hashmap<K, V>(header: &str, map: &HashMap<K, V>)
where
    K: Hash + std::cmp::Ord + std::fmt::Display,
    V: std::fmt::Display,
{
    let mut keys: Vec<&K> = map.keys().collect();
    keys.sort();

    print!("{} [", header);

    for &key in keys.iter() {
        if let Some(value) = map.get(key) {
            print!("{} => {}, ", key, value);
        }
    }
    println!("]");
}

fn process_input(inp: &str) {
    let sections: Vec<&str> = inp.split("\n\n").collect();

    let seeds: Vec<i32> = sections[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut seed2soil: HashMap<i32, i32> = HashMap::new();
    let mut soil2fertilizer: HashMap<i32, i32> = HashMap::new();
    let mut fertilizer2water: HashMap<i32, i32> = HashMap::new();
    let mut water2light: HashMap<i32, i32> = HashMap::new();
    let mut light2temperature: HashMap<i32, i32> = HashMap::new();
    let mut temperature2humidity: HashMap<i32, i32> = HashMap::new();
    let mut humidity2location: HashMap<i32, i32> = HashMap::new();

    for &section in &sections[1..] {
        if section.is_empty() {
            continue;
        }

        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut lines = section.lines();
        let header = lines.next().unwrap();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let v: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            for x in 0..v[2] {
                m.insert(v[1] + x, v[0] + x);
            }
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
    print!("Seeds =>[");
    seeds.iter().for_each(|t| print!(" {}", t));
    println!("]");

    print_hashmap("seed2soil", &seed2soil);
    print_hashmap("soil2fertilizer", &soil2fertilizer);
    print_hashmap("fertilizer2water", &fertilizer2water);
    print_hashmap("water2light", &water2light);
    print_hashmap("light2temperature", &light2temperature);
    print_hashmap("temperature2humidity", &temperature2humidity);
    print_hashmap("humidity2location", &humidity2location);

    let mut m = i32::MAX;
    for k in seeds {
        let l1 = seed2soil.get(&k).unwrap_or(&k);
        let l2 = soil2fertilizer.get(&l1).unwrap_or(&l1);
        let l3 = fertilizer2water.get(&l2).unwrap_or(&l2);
        let l4 = water2light.get(&l3).unwrap_or(&l3);
        let l5 = light2temperature.get(&l4).unwrap_or(&l4);
        let l6 = temperature2humidity.get(&l5).unwrap_or(&l5);
        let l7 = humidity2location.get(&l6).unwrap_or(&l6);

        if l7 < &m {
            m = *l7;
        }
        println!(
            "Chaining [ {} => {} => {} => {} => {} => {} => {} => {} ]",
            k, l1, l2, l3, l4, l5, l6, l7
        );
    }
    println!("-> {}", m);
}

pub fn lowest_location_number() {
    let mut contents = String::new();

    match File::open("data/d5_input.txt") {
        Ok(mut file) => {
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    //println!("Puzzle # 4.2: {}", no_of_matches_by_card_minus_one(&contents))
                    process_input(&contents);
                }
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}
