use lazy_static::lazy_static;
use regex::Regex;

use utils;

lazy_static!{
    pub static ref MUL_REGEX: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    
    // Sadly, RUST does not support lookback/lookahead
    // don't() block until a do() or line ending
    //pub static ref DONT_REGEX: Regex = Regex::new(r"don't\(\)(.+?)(?=do\(\)|$)").unwrap();
    
    pub static ref MUL_REGEX_STARTING_WITH_DO: Regex = Regex::new(r"do\(\)(.*)").unwrap();
}

fn get_muls(line: &str) -> u64 {
    MUL_REGEX
        .captures_iter(line)
        .map(|caps| {
            caps.get(1).unwrap().as_str().parse::<u64>().unwrap()
            * caps.get(2).unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum()
}

/*
fn get_donts(line: &str) -> u64 {
    DONT_REGEX
        .captures_iter(line)
        .map(|caps| caps.get(0).unwrap().as_str())
        .map(|s| get_muls(s))
        .sum()
}
        */

pub fn main_day3_task1() {
    let lines = utils::file_to_string_vector("day3\\data.txt");
    let result: u64 = lines.iter().map(|line| get_muls(line)).sum();
    println!("Day 3 task 1 result is {}", result);
}

/*
pub fn main_day3_task2() {
    let content = utils::file_to_string_vector("day3\\data.txt").concat();
    let muls: u64 = get_muls(content);
    let donts: u64 = get_donts(content);
    println!("Day 3 task 2 result is {}", muls - donts);
}
*/

pub fn main_day3_task2() {
    //Merge the whole file because a don't() can span multiple lines
    let content = utils::file_to_string_vector("day3\\data.txt").concat();
    let mut result = 0;
    
    //Split on dont'()
    let splitted = content.split("don't()").collect::<Vec<&str>>();
    
    //Take everything from the first split
    result += get_muls(splitted[0]);
    
    //In the remaining splits, find the first do(), and take everything after
    for s in splitted.iter().skip(1) {
        result += MUL_REGEX_STARTING_WITH_DO
            .find_iter(s)
            .map(|m| m.as_str())
            .map(|m| get_muls(m))
            .sum::<u64>();
    }
    
    println!("Day 3 task 1 result is {}", result);
}
