/*
-- Day 5: If You Give A Seed A Fertilizer ---
You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48
The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51
With this map, you can look up the soil number required for each initial seed number:

Seed number 79 corresponds to soil number 81.
Seed number 14 corresponds to soil number 14.
Seed number 55 corresponds to soil number 57.
Seed number 13 corresponds to soil number 13.
The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial seed numbers?

Your puzzle answer was 265018614.

--- Part Two ---
Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

seeds: 79 14 55 13
This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?

Your puzzle answer was 63179500.

Both parts of this puzzle are complete! They provide two gold stars: **
*/

use std::collections::{HashSet};
use std::fs::File;

use std::io::Read;

fn find_custom_range_lookup_old(range_map: &Vec<(u64, u64, u64)>, key: u64) -> u64 {
    let mut ret = key;
    for e in range_map {
        let r = e.0..(e.0 + e.2);

        if r.contains(&key) {
            ret = key - e.0 + e.1;
            return ret;
        }
    }
    ret
}

fn find_custom_range_lookup(range_map: &Vec<(u64, u64, u64)>, key: u64) -> u64 {
    range_map.iter()
        .find(|&&(start, _, length)| key >= start && key < start + length)
        .map_or(key, |&(start, offset, _)| key - start + offset)
}

fn extract_seeds_case_1(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn extract_seeds_case_2(line: &str) -> HashSet<u64> {
    let s = extract_seeds_case_1(line);

    let mut seeds: HashSet<u64> = HashSet::new();

    for chunk in s.chunks(2) {
        if let [s, r] = chunk {
            println!("{} {}", s, r);
            for e in (*s)..(*s + *r) {
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

        let mut m: Vec<(u64, u64, u64)> = vec![];
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
 
    seed2soil.sort_by(|a, b| a.0.cmp(&b.0));
    soil2fertilizer.sort_by(|a, b| a.0.cmp(&b.0));
    fertilizer2water.sort_by(|a, b| a.0.cmp(&b.0));
    water2light.sort_by(|a, b| a.0.cmp(&b.0));
    light2temperature.sort_by(|a, b| a.0.cmp(&b.0));
    humidity2location.sort_by(|a, b| a.0.cmp(&b.0));

    let mut m = u64::MAX;

    let s = extract_seeds_case_1(sections[0]);
    for chunk in s.chunks(2) {
        if let [s, r] = chunk {
            println!("\n{:20} {:20}", s, r);
            let mut i: u64 = 0;
            for k in (*s)..(*s + *r) {
                let l: u64;
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
                
                if i % 10000 == 0 {
                    print!("\r{:20} {:20} ", "Counting: ", i);
                } 
                i = i + 1;  
            }
            println!("\n{:20} {:20} {:20}", s, r, m);
        }
    }

    m
}

fn process_input(inp: &str, ranged_seeds: bool) -> u64 {
    let sections: Vec<&str> = inp.split("\n\n").collect();

    let seeds: HashSet<u64>;
    if ranged_seeds {
        seeds = extract_seeds_case_2(sections[0]);
    } else {
        let seeds_all = extract_seeds_case_1(sections[0]);
        seeds = seeds_all.into_iter().collect();
    }

    //println!("Number of seeds to process = {}", seeds.len());

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

        let mut m: Vec<(u64, u64, u64)> = vec![];
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
    let mut perc: u64 = 0;
    let mut i: u64 = 0;
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

        let new_perc = i / len * 100;
        if new_perc - perc > 10 {
            perc = new_perc;
            println!("% lookup done is {}", perc);
        }
        i = i + 1;
    }
    m
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d5_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 5.1: {}", process_input(&contents, false));
                println!("Puzzle # 5.2: {}", process_input_mod(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
