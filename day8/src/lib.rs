use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};

use env_logger;
use log;

use utils;

const FILENAME: &str = "day8\\data.txt";

fn get_data() -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();

    for (i, line) in utils::file_to_iter(FILENAME).enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' { continue }
            antennas.entry(c).or_insert(vec![]).push((i, j));
        }
    }

    antennas
}

pub fn main_day8_task1() {
    let _ = env_logger::try_init();

    let antennas = get_data();
    let size = utils::file_to_iter(FILENAME).next().unwrap().len();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, v) in antennas.iter() {
        log::debug!("{:?}", v);
        for i in 0..v.len()-1 {
            let p1 = v[i];
            for j in i+1..v.len() {
                let p2 = v[j];
                log::debug!("\t{} {} {} {}", p1.0, p1.1, p2.0, p2.1);
                if p2.0 <= 2*p1.0 && 2*p1.0 < p2.0+size && p2.1 <= 2*p1.1 && 2*p1.1 < p2.1+size {
                    antinodes.insert((2*p1.0-p2.0, 2*p1.1-p2.1));
                    log::debug!("\t\t{} {}", 2*p1.0-p2.0, 2*p1.1-p2.1);
                }
                if p1.0 <= 2*p2.0 && 2*p2.0 < p1.0+size && p1.1 <= 2*p2.1 && 2*p2.1 < p1.1+size {
                    antinodes.insert((2*p2.0-p1.0, 2*p2.1-p1.1));
                    log::debug!("\t\t{} {}", 2*p2.0-p1.0, 2*p2.1-p1.1);
                }
            }
        }
    }

    println!("Day 8 task 1 result is {}", antinodes.len());
}

pub fn main_day8_task2() {
    let _ = env_logger::try_init();

    let antennas = get_data();
    let size = utils::file_to_iter(FILENAME).next().unwrap().len();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, v) in antennas.iter() {
        log::debug!("{:?}", v);
        for i in 0..v.len()-1 {
            let p1 = v[i];
            for j in i+1..v.len() {
                let p2 = v[j];
                let x_distance = max(p1.0, p2.0) - min(p1.0, p2.0);
                let y_distance = max(p1.1, p2.1) - min(p1.1, p2.1);
                log::debug!("\t{} {} {} {} {} {}", p1.0, p1.1, p2.0, p2.1, x_distance, y_distance);

                if (p1.0 > p2.0 && p1.1 > p2.1) || (p1.0 < p2.0 && p1.1 < p2.1) {
                    //Slope right up
                    for i in 0.. {
                        if p1.0 >= i*x_distance && p1.1 >= i*y_distance {
                            antinodes.insert((p1.0-i*x_distance, p1.1-i*y_distance));
                            log::debug!("\t\t{} {}", p1.0-i*x_distance, p1.1-i*y_distance);
                        }
                        else { break }
                    }
                    for i in 1.. {
                        if p1.0+i*x_distance < size && p1.1+i*y_distance < size {
                            antinodes.insert((p1.0+i*x_distance, p1.1+i*y_distance));
                            log::debug!("\t\t{} {}", p1.0+i*x_distance, p1.1+i*y_distance);
                        }
                        else { break }
                    }
                }
                else {
                    //Slope left up
                    for i in 0.. {
                        if p1.0 >= i*x_distance && p1.1+i*y_distance < size {
                            antinodes.insert((p1.0-i*x_distance, p1.1+i*y_distance));
                            log::debug!("\t\t{} {}", p1.0-i*x_distance, p1.1+i*y_distance);
                        }
                        else { break }
                    }
                    for i in 1.. {
                        if p1.0+i*x_distance < size && p1.1 >= i*y_distance {
                            antinodes.insert((p1.0+i*x_distance, p1.1-i*y_distance));
                            log::debug!("\t\t{} {}", p1.0+i*x_distance, p1.1-i*y_distance);
                        }
                        else { break }
                    }
                }
            }
        }
    }

    for a in antinodes.iter() {
        log::debug!("{:?}", a);
    }

    println!("Day 8 task 2 result is {}", antinodes.len());
}
