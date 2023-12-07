/*
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum HandType {
    FiveOfKind = 1,
    FourOfkind = 2,
    FullHouse = 3,
    ThreeOfkind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfKind => 1,
            HandType::FourOfkind => 2,
            HandType::FullHouse => 3,
            HandType::ThreeOfkind => 4,
            HandType::TwoPair => 5,
            HandType::OnePair => 6,
            HandType::HighCard => 7,
        }
    }
}

fn hand_type(h: &str) -> HandType {
    let c = h.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    });

    let m = c.iter().map(|(_, count)| count).max().unwrap_or(&0);

    let ret: HandType;
    match m {
        5 => HandType::FiveOfKind,
        4 => HandType::FourOfkind,
        3 => {
            if c.into_iter().any(|(_, count)| count == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfkind
            }
        }
        2 => {
            let mut n = 0;
            for e in c.into_iter() {
                if e.1 == 2 {
                    n += 1;
                }
            }
            if n == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => HandType::HighCard,
    }
}

fn process_input_lines(haystack: &str) -> u32 {
    let mut hands: Vec<(&str, u32)> = vec![];

    for hand in haystack.lines() {
        let entry: Vec<&str> = hand.split_whitespace().collect();
        let hand = (entry[0], entry[1].parse::<u32>().unwrap());
        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        println!("{:?} {:?}", a, b);
        let a_t = hand_type(a.0);
        let b_t = hand_type(b.0);

        println!("{:?} {:?}", a_t, b_t);

        if a_t.value() != b_t.value() {
            return a_t.value().cmp(&b_t.value());
        }

        let p: HashMap<char, u32> = HashMap::from([
            ('A', 1),
            ('K', 2),
            ('Q', 3),
            ('J', 4),
            ('T', 5),
            ('9', 6),
            ('8', 7),
            ('7', 8),
            ('6', 9),
            ('5', 10),
            ('4', 11),
            ('3', 12),
            ('2', 13),
        ]);
        for i in 0..5 {
            let l = p.get(&a.0.chars().nth(i).unwrap()).unwrap();
            let r = p.get(&b.0.chars().nth(i).unwrap()).unwrap();
            if l != r {
                return l.cmp(r);
            }
        }
        println!("Hello haaaaaaaaaaaaa");
        1.cmp(&2)
    });
    hands.reverse();

    let mut sum = 0;
    for i in 0..hands.len() {
        sum = sum + hands[i].1 * ((i + 1) as u32);
    }

    println!("{:?}", hands);

    sum
}

fn hand_type2(h: &str) -> HandType {
    let c = h
        .chars()
        .fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
        .clone();

    let m = c.iter().map(|(_, count)| count).max().unwrap_or(&0);

    let ret: HandType;
    match m {
        5 => HandType::FiveOfKind,
        4 => {
            if h.contains('J') {
                HandType::FiveOfKind
            } else {
                HandType::FourOfkind
            }
        }
        3 => {
            if c.iter().any(|(_, count)| count == &2) {
                match c.get(&'J') {
                    Some(v) => match v {
                        3 => HandType::FiveOfKind,
                        2 => HandType::FiveOfKind,
                        1 => HandType::FourOfkind,
                        _ => HandType::FullHouse,
                    },
                    None => HandType::FullHouse,
                }
            } else {
                return match c.get(&'J') {
                    Some(v) => {
                        return match v {
                            3 => HandType::FourOfkind,
                            1 => HandType::FourOfkind,
                            _ => HandType::ThreeOfkind,
                        };
                    }
                    None => HandType::ThreeOfkind,
                };
            }
        }
        2 => {
            let mut n = 0;
            for e in c.iter() {
                if e.1 == &2 {
                    n += 1;
                }
            }
            if n == 2 {
                return match c.get(&'J') {
                    Some(v) => {
                        return match v {
                            2 => HandType::FourOfkind,
                            1 => HandType::FullHouse,
                            _ => HandType::TwoPair,
                        };
                    }
                    None => HandType::TwoPair,
                };
            } else {
                return match c.get(&'J') {
                    Some(v) => {
                        return match v {
                            2 => HandType::ThreeOfkind,
                            1 => HandType::ThreeOfkind,
                            _ => HandType::OnePair,
                        };
                    }
                    None => HandType::OnePair,
                };
            }
        }
        1 => {
            return match c.get(&'J') {
                Some(v) => {
                    return match v {
                        1 => HandType::OnePair,
                        _ => HandType::HighCard,
                    };
                }
                None => HandType::HighCard,
            };
        }
        _ => HandType::HighCard,
    }
}

fn process_input_lines2(haystack: &str) -> u32 {
    let mut hands: Vec<(&str, u32)> = vec![];

    for hand in haystack.lines() {
        let entry: Vec<&str> = hand.split_whitespace().collect();
        let hand = (entry[0], entry[1].parse::<u32>().unwrap());
        hands.push(hand);
    }

    hands.sort_by(|a, b| {
        println!("{:?} {:?}", a, b);
        let a_t = hand_type2(a.0);
        let b_t = hand_type2(b.0);

        println!("{:?} {:?}", a_t, b_t);

        if a_t.value() != b_t.value() {
            return a_t.value().cmp(&b_t.value());
        }

        let p: HashMap<char, u32> = HashMap::from([
            ('A', 1),
            ('K', 2),
            ('Q', 3),
            ('J', 14),
            ('T', 5),
            ('9', 6),
            ('8', 7),
            ('7', 8),
            ('6', 9),
            ('5', 10),
            ('4', 11),
            ('3', 12),
            ('2', 13),
        ]);
        for i in 0..5 {
            let l = p.get(&a.0.chars().nth(i).unwrap()).unwrap();
            let r = p.get(&b.0.chars().nth(i).unwrap()).unwrap();
            if l != r {
                return l.cmp(r);
            }
        }
        println!("Hello haaaaaaaaaaaaa");
        1.cmp(&2)
    });
    hands.reverse();

    let mut sum = 0;
    for i in 0..hands.len() {
        sum = sum + hands[i].1 * ((i + 1) as u32);
    }

    println!("{:?}", hands);

    sum
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d7_input.txt") {
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
