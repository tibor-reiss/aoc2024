use itertools::Itertools;
use regex::Regex;

use utils;

fn get_data() -> Vec<(u64, Vec<u64>)> {
    let mut data: Vec<(u64, Vec<u64>)> = vec![];
    let re = Regex::new(r"\d+").unwrap();
    for line in utils::file_to_iter("day7\\data.txt") {
        for (i, part) in line.split(":").enumerate() {
            match i {
                0 => data.push((part.parse::<u64>().unwrap(), vec![])),
                1 => {
                    if let Some((_, last)) = data.last_mut() {
                        *last = re
                            .find_iter(part)
                            .map(|nr| nr.as_str().parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();
                        }
                },
                _ => (),
            }
        }
    }
    data
}

fn is_correct_part1(expected: u64, numbers: &[u64]) -> bool {
    let a = vec!['*', '+'];
    for ops in (1..numbers.len()).map(|_| a.iter()).multi_cartesian_product()  {
        let mut result = numbers[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                '*' => result *= numbers[i+1],
                '+' => result += numbers[i+1],
                _ => (),
            }
        }
        if result == expected { return true; }
    }
    false
}

fn is_correct_part2(expected: u64, numbers: &[u64]) -> bool {
    let a = vec!["*", "+", "||"];
    for ops in (1..numbers.len()).map(|_| a.iter()).multi_cartesian_product()  {
        let mut result = numbers[0];
        for (i, &&op) in ops.iter().enumerate() {
            match op {
                "*" => result *= numbers[i+1],
                "+" => result += numbers[i+1],
                "||" => result = result * (10 as u64).pow(numbers[i+1].ilog(10) + 1) + numbers[i+1],
                _ => panic!("Invalid operator"),
            }
        }
        if result == expected { return true; }
    }
    false
}

pub fn main_day7_task1() {
    let data = get_data();
    
    let result: u64 = data
        .iter()
        .map(|(expected, numbers)| if is_correct_part1(*expected, &numbers) { *expected } else { 0 })
        .sum();
    println!("Day 7 task 1 result is {}", result);
}

pub fn main_day7_task2() {
    let data = get_data();
    
    let result: u64 = data
        .iter()
        .map(|(expected, numbers)| if is_correct_part2(*expected, &numbers) { *expected } else { 0 })
        .sum();
    println!("Day 7 task 2 result is {}", result);
}
