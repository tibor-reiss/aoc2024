use std::collections::{HashMap, HashSet};

use env_logger;
use log;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;
use queues::*;

use utils;

lazy_static!{
    pub static ref COMPS: Regex = Regex::new(r"(?<comp1>[a-z]{2})-(?<comp2>[a-z]{2})").unwrap();
}

fn get_data(filename: &str) -> (HashMap<String, HashSet<String>>, HashSet<String>) {
    let mut computers: HashMap<String, HashSet<String>> = HashMap::new();
    let mut candidates: HashSet<String> = HashSet::new();
    
    for line in utils::file_to_iter(filename) {
        let caps = COMPS.captures(&line).unwrap();
        let mut comp1 = caps["comp1"].to_string();
        let mut comp2 = caps["comp2"].to_string();
        if comp1 > comp2 { (comp1, comp2) = (comp2, comp1); }
        if comp1.starts_with("t") {
            candidates.insert(comp1.clone());
            computers.entry(comp1).or_insert(HashSet::new()).insert(comp2);
        }
        else if comp2.starts_with("t") {
            candidates.insert(comp2.clone());
            computers.entry(comp2).or_insert(HashSet::new()).insert(comp1);
        }
        else {
            computers.entry(comp1).or_insert(HashSet::new()).insert(comp2);
        }
    }

    (computers, candidates)
}

fn get_data2(filename: &str) -> (HashMap<String, HashSet<String>>, HashSet<String>) {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut computers: HashSet<String> = HashSet::new();
    
    for line in utils::file_to_iter(filename) {
        let caps = COMPS.captures(&line).unwrap();
        let comp1 = caps["comp1"].to_string();
        let comp2 = caps["comp2"].to_string();
        computers.insert(comp1.clone());
        computers.insert(comp2.clone());
        // Add both for faster lookup
        connections.entry(comp1.clone()).or_insert(HashSet::new()).insert(comp2.clone());
        connections.entry(comp2).or_insert(HashSet::new()).insert(comp1.clone());
    }

    (connections, computers)
}

fn set_to_string(s: &HashSet<String>) -> String {
    s.iter().sorted().join(",")
}

pub fn main_day23_task1() {
    _ = env_logger::builder().format_timestamp(None).try_init();

    let (computers, candidates) = get_data("day23\\data.txt");
    log::debug!("{:?}", computers);
    log::debug!("{:?}", candidates);

    let mut result = 0;
    for comp1 in candidates.iter() {
        log::debug!("{}", comp1);
        if !computers.contains_key(comp1) { continue }
        for (comp2, comp3) in computers.get(comp1).unwrap().iter().tuple_combinations() {
            log::debug!("\t{} {}", comp2, comp3);
            if computers.contains_key(comp2) && computers.get(comp2).unwrap().contains(comp3) {
                result += 1;
                continue
            }
            if computers.contains_key(comp3) && computers.get(comp3).unwrap().contains(comp2) {
                result += 1;
                continue
            }
        }
    }
    println!("Day 23 task 1 result is {}", result);
}

pub fn main_day23_task2() {
    _ = env_logger::builder().format_timestamp(None).try_init();

    let (connections, computers) = get_data2("day23\\data.txt");
    log::debug!("{:?}", computers);

    // Current lan, possible extensions
    let mut q: Queue<(HashSet<String>, HashSet<String>)> = queue![];
    for comp in computers.iter() {
        let mut s = HashSet::new();
        s.insert(comp.clone());
        let _ = q.add((s, connections.get(comp).unwrap().clone()));
    }
    let mut max_length = 1;
    let mut max_set: String = String::from("");
    let mut seen = HashSet::new();
    loop {
        if let Ok((current_lan, extensions)) = q.remove() {
            for comp in extensions.iter() {
                let new_extensions: HashSet<String> = extensions.intersection(connections.get(comp).unwrap()).cloned().collect();
                let mut new_s = current_lan.clone();
                new_s.insert(comp.clone());
                let pattern = set_to_string(&new_s);
                if seen.contains(&pattern) {
                    continue
                }
                else {
                    seen.insert(pattern.clone());
                }
                //log::debug!("{:?} {:?}", new_s, new_extensions);
                if new_s.len() > max_length {
                    max_length = new_s.len();
                    max_set = pattern;
                    log::debug!("length={}", max_length);
                }
                if !new_extensions.is_empty() {
                    let _ = q.add((new_s, new_extensions));
                }
            }
        }
        else { break }
    }

    println!("Day 23 task 2 result is {} with length of {}", max_set, max_length);
}
