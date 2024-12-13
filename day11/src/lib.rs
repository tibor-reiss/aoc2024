use std::collections::HashMap;

use env_logger;
use log;

use utils;

const FILENAME: &str = "day11\\data.txt";

fn get_data() -> Vec<u64> {
    utils::file_to_string_vector(FILENAME)[0].split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
}

fn transform(stone: u64) -> Vec<u64> {
    // Rule 1: 0 to 1
    if stone == 0 { return vec![1] }
    
    // Rule 2: split if length is even
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        return vec![
            stone_str[0..stone_str.len()/2].parse::<u64>().unwrap(),
            stone_str[stone_str.len()/2..].parse::<u64>().unwrap(),
        ]
    }

    // Rule 3: multiply by 2024
    vec![stone * 2024]
}

pub fn main_day11_task1() {
    _ = env_logger::try_init();
    
    let mut stones = get_data();
    log::debug!("{stones:?}");

    for i in 1..=25 {
        stones = stones.iter().map(|&stone| transform(stone)).flatten().collect();
    }

    println!("Day 11 task 1 result is {}", stones.len());
}

pub fn main_day11_task2() {
    _ = env_logger::try_init();
    
    let mut stones = get_data();
    log::debug!("{stones:?}");

    // Stone number, number of occurrence
    let mut total_map: HashMap<u64, u64> = HashMap::new();
    for v in stones {
        *total_map.entry(v).or_insert(0) += 1;
    }

    for i in 1..=75 {
        let mut temp_map: HashMap<u64, u64> = HashMap::new();
        for (stone, occurrence) in total_map {
            for s in transform(stone) {
                temp_map
                    .entry(s)
                    .and_modify(|e| *e += occurrence)
                    .or_insert(occurrence);
            }
        }
        total_map = temp_map.clone();
    }

    println!("Day 11 task 2 result is {}", total_map.values().sum::<u64>());
}
