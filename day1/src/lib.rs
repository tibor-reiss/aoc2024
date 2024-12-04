use std::{collections::HashMap, hash::Hash};

use utils;

fn get_content() -> (Vec<i64>, Vec<i64>) {
    let lines = utils::file_to_string_vector("day1\\data.txt");
    let mut col1 = vec![];
    let mut col2 = vec![];

    for line in lines {
        let parts: Vec<i64> = line
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        col1.push(parts[0]);
        col2.push(parts[1]);
    }
    col1.sort();
    col2.sort();
    (col1, col2)
}

pub fn main_day1_task1() {
    let (col1, col2) = get_content();
    let result: i64 = col1.iter().zip(col2.iter()).map(|(value1, value2)| (value1 - value2).abs()).sum();
    println!("Day 1 task 1 result is {}", result);
}

pub fn main_day1_task2() {
    let (col1, col2) = get_content();
    let mut col2_map: HashMap<i64, i64> = HashMap::new();
    
    for value in col2 {
        *col2_map.entry(value).or_insert(0) += 1;
    }
    
    let result: i64 = col1.iter().map(|value| col2_map.get(value).unwrap_or(&0) * value).sum();
    println!("Day 1 task 1 result is {}", result);
}
