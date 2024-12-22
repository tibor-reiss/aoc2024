use std::collections::{HashMap, HashSet};

use env_logger;
use log;

use utils;

fn get_data(filename: &str) -> Vec<i64> {
    utils::file_to_iter(filename)
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn get_secret_number(nr: i64) -> i64 {
    let mut nr2= nr * 64;
    let mut nr3;

    nr2 ^= nr;
    nr2 %= 16777216;
    nr3 = nr2;

    nr2 /= 32;
    nr2 ^= nr3;
    nr2 %= 16777216;
    nr3 = nr2;

    nr2 *= 2048;
    nr2 ^= nr3;
    nr2 %= 16777216;

    nr2
}

fn get_sequence_map(nr: i64, iterations: usize, all_keys: &mut HashSet<(i64, i64, i64, i64)>) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut smap = HashMap::new();
    let mut nr2 = nr;
    let mut endings = vec![];
    endings.push(nr2 % 10);
    let mut diffs = vec![];

    for _ in 0..iterations {
        nr2 = get_secret_number(nr2);
        endings.push(nr2 % 10);
        diffs.push(endings[endings.len() - 1] - endings[endings.len() - 2]);
    }

    for i in 0..iterations-4 {
        let changes = (diffs[i], diffs[i+1], diffs[i+2], diffs[i+3]);
        smap.entry(changes).or_insert(endings[i+4]);
        all_keys.insert(changes);
    }

    smap
}

pub fn main_day22_task1() {
    _ = env_logger::builder().format_timestamp(None).try_init();
    
    let nrs = get_data("day22\\data.txt");
    let mut total = 0;
    for nr in nrs.iter() {
        let mut nr2 = *nr;
        for _ in 0..2000 {
            nr2 = get_secret_number(nr2);
        }
        total += nr2;
    }

    println!("Day 22 task 1 result is {}", total);
}

pub fn main_day22_task2() {
    _ = env_logger::builder().format_timestamp(None).try_init();
    
    let nrs = get_data("day22\\data.txt");
    let mut all_keys = HashSet::new();
    let mut all_smaps = vec![];
    for nr in nrs.iter() {
        all_smaps.push(get_sequence_map(*nr, 2000, &mut all_keys));
    }
    log::debug!("{}", all_smaps.len());
    let mut max_result: i64 = 0;
    for k in all_keys.iter() {
        let mut result = 0;
        for smap in all_smaps.iter() {
            if let Some(v) = smap.get(k) {
                result += v;
            }
        }
        if result > max_result {
            max_result = result;
            log::debug!("{:?}", k);
        }
    }
    println!("Day 22 task 2 result is {}", max_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nrs = get_data("data_sample.txt");
        let mut total = 0;
        for nr in nrs.iter() {
            let mut nr2 = *nr;
            for _ in 0..2000 {
                nr2 = get_secret_number(nr2);
            }
            total += nr2;
        }
        assert_eq!(total, 37327623);
    }

    #[test]
    fn test21() {
        let mut all_keys = HashSet::new();
        let smap = get_sequence_map(123, 9, &mut all_keys);
        assert_eq!(*smap.get(&(-1,-1,0,2)).unwrap(), 6);
    }

    #[test]
    fn test22() {
        let nrs = get_data("data_sample2.txt");
        let mut all_keys = HashSet::new();
        let mut all_smaps = vec![];
        
        for nr in nrs.iter() {
            all_smaps.push(get_sequence_map(*nr, 2000, &mut all_keys));
        }
        
        let mut max_result: i64 = 0;
        for k in all_keys.iter() {
            let mut result = 0;
            for smap in all_smaps.iter() {
                if let Some(v) = smap.get(k) {
                    result += v;
                }
            }
            if result > max_result {
                max_result = result;
            }
        }

        assert_eq!(max_result, 23);
    }
}
