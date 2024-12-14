use std::collections::{HashMap, HashSet};

use env_logger;
use log;

use utils;

const FILENAME: &str = "day10\\data.txt";
const RADIX: u32 = 10;

fn get_data() -> Vec<Vec<u32>> {
    let mut height_map = vec![];
    
    for line in utils::file_to_iter(FILENAME) {
        height_map.push(line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect())
    }

    height_map
}

fn calc_trail_map(height_map: &Vec<Vec<u32>>) -> Vec<HashMap<(usize, usize), Vec<(usize, usize)>>> {
    // Make a map of where one can go from a given position
    // Rules:
    //  1. only horizontal or vertical, not diagonal
    //  2. next number must be exactly one greater

    let mut trail_map: Vec<HashMap<(usize, usize), Vec<(usize, usize)>>> = vec![];
    for _ in 0..=9 {
        trail_map.push(HashMap::new());
    }
    let x_dim = height_map.len();
    let y_dim = height_map[0].len();

    for (i, row) in height_map.iter().enumerate() {
        for (j, nr) in row.iter().enumerate() {
            let mut neighbours = vec![];
            // Above
            if 0 < i {
                if height_map[i-1][j] == nr + 1 { neighbours.push((i-1, j)) }
            }
            // Below
            if i < x_dim -1 {
                if height_map[i+1][j] == nr + 1 { neighbours.push((i+1, j)) }
            }
            // Left
            if 0 < j {
                if height_map[i][j-1] == nr + 1 { neighbours.push((i, j-1)) }
            }
            // Right
            if j < y_dim -1 {
                if height_map[i][j+1] == nr + 1 { neighbours.push((i, j+1)) }
            }
            trail_map[*nr as usize].insert((i, j), neighbours);
        }
    }
    log::debug!("{:?}", trail_map[9]);

    trail_map
}

fn calc_reachable_tops(trail_map: &Vec<HashMap<(usize, usize), Vec<(usize, usize)>>>) -> usize {
    // Calculate all the tops which can be reached - no duplicates
    let mut reachable = HashSet::new();

    for i in 0..=8 {
        reachable.clear();
        for (_, neigbours) in trail_map[i].iter() {
            for neigbour in neigbours {
                reachable.insert(neigbour.clone());
            }
        }
    }
    
    reachable.len()
}

fn calc_trailhead_scores(trail_map: &Vec<HashMap<(usize, usize), Vec<(usize, usize)>>>) -> usize {
    // Calculate number of tops which can be reached from a starting point, and sum up
    let mut total_trailhead_score = 0;

    for (k, neighbours) in trail_map[0].iter() {
        log::debug!("{:?} {:?}", k, neighbours);
        let mut starting_points: HashSet<(usize, usize)> = HashSet::new();
        for n in neighbours.iter() {
            starting_points.insert(n.clone());
        }
        
        for i in 1..=8 {
            log::debug!("\tstarting_point: {:?}", starting_points);
            let mut reachable: HashSet<(usize, usize)> = HashSet::new();
            for s in starting_points {
                for neighbour in trail_map[i][&s].iter() {
                    reachable.insert(neighbour.clone());
                }
            }
            starting_points = reachable.clone();
        }
        total_trailhead_score += starting_points.len();
    }
    
    total_trailhead_score
}

fn calc_all_trails(trail_map: &Vec<HashMap<(usize, usize), Vec<(usize, usize)>>>) -> u32 {
    let mut trail_score = HashMap::new();
    for i in (0..=9).rev() {
        for (k, v) in trail_map[i].iter() {
            let mut score: u32 = 0;
            if i == 9 {
                score = 1;
            } else {
                for neighbour in v.iter() {
                    score += trail_score[neighbour];
                }
            }
            trail_score.insert(k, score);
            log::debug!("{i} {k:?} {score}");
        }   
    }
    log::debug!("{:?}", trail_map[0]);

    trail_map[0].keys().map(|p| trail_score[p]).sum()
}

pub fn main_day10_task1() {
    _ = env_logger::try_init();

    let height_map = get_data();
    log::debug!("{height_map:?}");

    let trail_map = calc_trail_map(&height_map);
    println!("Day 10 task 1 result is {}", calc_trailhead_scores(&trail_map));
    println!("Day 10 extra - all reachable tops, no duplicates is {}", calc_reachable_tops(&trail_map));
}

pub fn main_day10_task2() {
    _ = env_logger::try_init();

    let height_map = get_data();
    log::debug!("{height_map:?}");

    let trail_map = calc_trail_map(&height_map);
    println!("Day 10 task 2 result is {}", calc_all_trails(&trail_map));
}
