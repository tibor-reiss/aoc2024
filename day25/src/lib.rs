use env_logger;
use log;
use itertools::Itertools;

use utils;

fn get_data(filename: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks: Vec<Vec<u8>> = vec![];
    let mut keys: Vec<Vec<u8>> = vec![];
    let mut is_lock: bool;
    
    let stream = utils::file_to_iter(filename);
    for lines in &stream.chunks(8) {
        is_lock = false;
        let mut height_map: Vec<u8> = vec![0; 5];
        for (row, line) in lines.enumerate() {
            if row == 0 {
                if line.contains("#####") { is_lock = true; }
            }
            else if row == 6 { continue }
            else {
                for (j, col) in line.chars().enumerate() {
                    match col {
                        '#'  => height_map[j] += 1,
                        _ => (),
                    }
                }
            }
        }
        if is_lock { locks.push(height_map); }
        else { keys.push(height_map); }
    }

    (keys, locks)
}

fn count_fits(locks: &Vec<Vec<u8>>, keys: &Vec<Vec<u8>>) -> u32 {
    let mut fit = 0;
    for l in locks.iter() {
        for k in keys.iter() {
            let mut is_fit = true;
            for col in 0..k.len() {
                if l[col] + k[col] > 5 {
                    is_fit = false;
                    break;
                }
            }
            if is_fit { fit += 1; }
        }
    }

    fit
}

pub fn main_day25_task1() {
    _ = env_logger::builder().format_timestamp(None).try_init();

    let (keys, locks) = get_data("day25\\data.txt");
    log::debug!("LOCKS");
    for l in locks.iter() {
        log::debug!("{:?}", l);
    }
    log::debug!("KEYS");
    for k in keys.iter() {
        log::debug!("{:?}", k);
    }

    let fit = count_fits(&locks, &keys);
    println!("Day 25 task 1 result is {}", fit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (keys, locks) = get_data("data_sample.txt");
        let fit = count_fits(&locks, &keys);
        assert_eq!(fit, 3);
    }
}
