/*
*/

use evalexpr::*;
use regex::Regex;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::util::util;
use num::integer::gcd;

#[derive(Debug)]
struct Module {
    name: String,
    mod_type: char,
    //out_mod: Vec<Rc<RefCell<Module>>>,
    out_names: Vec<String>,
    memory: HashMap<String, bool>,
}

fn parse_module(input: &str) -> Option<Module> {
    let parts: Vec<&str> = input.trim().split("->").take(2).collect();
    if parts.len() != 2 {
        return None;
    }

    let mut mem: HashMap<String, bool> = HashMap::new();
    let (t1, t2) = (parts[0].trim(), parts[1].trim());

    let (name, mod_type) = if t1 == "broadcaster" {
        (t1.to_owned(), 'b')
    } else {
        let mod_type = t1.chars().next()?;
        (t1[1..].to_owned(), mod_type)
    };

    if mod_type == '%' {
        mem.insert("state".to_string(), false);
    }

    let out_names: Vec<String> = t2.split(',').map(str::trim).map(String::from).collect();

    Some(Module {
        name,
        mod_type,
        //out_mod: Vec::new(),
        out_names,
        memory: mem.clone(),
    })
}

fn process_input_lines(input: &str) -> i64 {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        if let Some(m) = parse_module(line) {
            modules.insert(m.name.clone(), m);
        }
    }

    //println!("Modules {:?}", modules);
    let mut updates = Vec::new();
    for m in modules.values() {
        for o in &m.out_names {
            if let Some(t) = modules.get(o) {
                if t.mod_type == '&' {
                    updates.push((o.clone(), m.name.clone()));
                }
            }
        }
    }

    for (o, m_name) in updates {
        if let Some(t) = modules.get_mut(&o) {
            t.memory.insert(m_name, false);
        }
    }

    let mut low: i64 = 0;
    let mut hi: i64 = 0;
    for _ in 0..1000 {
        low += 1;
        let mut q: VecDeque<(String, String, bool)> = VecDeque::new();
        if let Some(b) = modules.get("broadcaster") {
            for t in &b.out_names {
                q.push_back((b.name.to_string(), t.to_string(), false));
            }
        }

        //println!("Q {:?}", q );
        while let Some((src, dest, pulse)) = q.pop_front() {
            let pp = if pulse { "hi" } else { "lo" };
            //println!("{:?} --{:?}-> {:?} ({:?}, {:?})", src, pp, dest, low, hi);
            if pulse {
                hi += 1
            } else {
                low += 1
            }

            if let Some(dest_node) = modules.get_mut(&dest) {
                if dest_node.mod_type == '%' {
                    if !pulse {
                        if let Some(s) = dest_node.memory.get("state") {
                            let ns = !s;
                            dest_node.memory.insert("state".to_string(), ns);

                            for t in &dest_node.out_names {
                                q.push_back((dest_node.name.to_string(), t.to_string(), ns));
                            }
                        }
                    }
                } else {
                    dest_node.memory.insert(src, pulse);
                    //println!("{:?}", dest_node);
                    let np = !dest_node.memory.values().all(|p| *p);
                    for t in &dest_node.out_names {
                        q.push_back((dest_node.name.to_string(), t.to_string(), np));
                    }
                }
            } else {
                continue;
            }
        }
    }

    low * hi
}


fn process_input_lines2(input: &str) -> usize {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        if let Some(m) = parse_module(line) {
            modules.insert(m.name.clone(), m);
        }
    }

    //println!("Modules {:?}", modules);
    let mut updates = Vec::new();
    for m in modules.values() {
        for o in &m.out_names {
            if let Some(t) = modules.get(o) {
                if t.mod_type == '&' {
                    updates.push((o.clone(), m.name.clone()));
                }
            }
        }
    }

    for (o, m_name) in updates {
        if let Some(t) = modules.get_mut(&o) {
            t.memory.insert(m_name, false);
        }
    }

    
    //println!("Modules {:?}", modules);

    let rx_feeders: Vec<_> = modules.iter().filter(|(_,m)|{
        m.out_names.contains(&"rx".to_string())
    }).map(|(e,_)|{
        e
    }).collect();

    //println!("RX Feeder  = {:?}", rx_feeders);
    let rx_feeder = rx_feeders.get(0).unwrap().to_string();

    let rx_feeder_minus_1_visit_keys: Vec<_> = modules.iter().filter(|(_, m)| {
        m.out_names.contains(&rx_feeder)
    }).map(|(e, _)| e.clone()).collect();

    let mut rx_feeder_minus_1_visit: HashMap<_, _> = rx_feeder_minus_1_visit_keys.into_iter().map(|e| (e, false)).collect();

    let mut rx_feeder_cycles:HashMap<String, usize> = HashMap::new(); 
    let mut low: i64 = 0;
    let mut hi: i64 = 0;

    let mut pulse_counter = 0 as usize;

    'outer:for _ in 0..10000 {
        pulse_counter += 1;
        low += 1;
        let mut q: VecDeque<(String, String, bool)> = VecDeque::new();
        if let Some(b) = modules.get("broadcaster") {
            for t in &b.out_names {
                q.push_back((b.name.to_string(), t.to_string(), false));
            }
        }

        //println!("Q {:?}", q );
        while let Some((src, dest, pulse)) = q.pop_front() {
            let pp = if pulse { "hi" } else { "lo" };
            //println!("{:?} --{:?}-> {:?} ({:?}, {:?})", src, pp, dest, low, hi);
            if pulse {
                hi += 1
            } else {
                low += 1
            }

            match dest == rx_feeder && pulse {
                true => { 
                    let s = src.to_string();
                    if let Some(value) = rx_feeder_minus_1_visit.get_mut(&s) {
                        *value = true;
                    }

                    rx_feeder_cycles.entry(s).or_insert(pulse_counter);

                   // println!("RX Feeder - 1 {:?} ", rx_feeder_minus_1_visit);
                    //println!("RX Feeder Cycles- 1 {:?} ", rx_feeder_cycles);

                    if rx_feeder_minus_1_visit.iter().all(|(_, visted)| { *visted } ) {
                        let mut c: usize = 1;
                        for t in rx_feeder_cycles.values() {
                            c = c * (t/gcd(c, *t)) 
                        }

                        return c;

                    }
                }
                false => (),
            }

            if let Some(dest_node) = modules.get_mut(&dest) {
                if dest_node.mod_type == '%' {
                    if !pulse {
                        if let Some(s) = dest_node.memory.get("state") {
                            let ns = !s;
                            dest_node.memory.insert("state".to_string(), ns);

                            for t in &dest_node.out_names {
                                q.push_back((dest_node.name.to_string(), t.to_string(), ns));
                            }
                        }
                    }
                } else {
                    dest_node.memory.insert(src, pulse);
                    //println!("{:?}", dest_node);
                    let np = !dest_node.memory.values().all(|p| *p);
                    for t in &dest_node.out_names {
                        q.push_back((dest_node.name.to_string(), t.to_string(), np));
                    }
                }
            } else {
                continue;
            }
        }
    }

    0
}

pub fn play() {
    let mut contents = String::new();

    match File::open("data/d20_input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Puzzle # 19.1: {}", process_input_lines(&contents));
                println!("Puzzle # 19.2: {}", process_input_lines2(&contents));
            }
            Err(e) => println!("Error reading file: {}", e),
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
