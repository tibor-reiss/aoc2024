use std::collections::HashMap;

use env_logger;
use log;

use utils;

const FILENAME: &str = "day19\\data.txt";

fn get_data() -> (Vec<String>, Vec<String>) {
    let towels = utils::file_to_iter(FILENAME)
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let patterns = utils::file_to_iter(FILENAME)
        .skip(2)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (towels, patterns)
}

fn is_valid(pattern: &str, towels: &Vec<String>) -> bool {
    let t = towels
        .iter()
        .filter(|&s| pattern.contains(s))
        .cloned()
        .collect::<Vec<String>>();

    for i in t.iter() {
        if i == pattern { return true }
        if *i == pattern[0..i.len()] { if is_valid(&pattern[i.len()..], &t) { return true } }
    }

    false
}

fn is_valid_with_count(pattern: &str, towels: &Vec<String>, counts: &mut HashMap<String, u64>) -> u64 {
    if let Some(counter) = counts.get(pattern) { return *counter; }
    
    let mut counter = 0;
    let t = towels
        .iter()
        .filter(|&s| pattern.contains(s))
        .cloned()
        .collect::<Vec<String>>();

    for i in t.iter() {
        if i == pattern {
            counter += 1;
            continue
        }
        if *i == pattern[0..i.len()] {
            counter += is_valid_with_count(&pattern[i.len()..], &t, counts);
        }
    }

    counts.insert(pattern.to_string(), counter);
    counter
}

pub fn main_day19_task1() {
    _ = env_logger::try_init();

    let (towels, patterns) = get_data();

    println!("Day 19 task 1 result is {}", patterns.iter().filter(|p| is_valid(&p, &towels)).count());
}

pub fn main_day19_task2() {
    _ = env_logger::try_init();

    let (towels, patterns) = get_data();
    let mut counts = HashMap::new();

    println!("Day 19 task 2 result is {}", patterns.iter().map(|p| is_valid_with_count(&p, &towels, &mut counts)).sum::<u64>());
    log::debug!("{}", counts.len());
}
