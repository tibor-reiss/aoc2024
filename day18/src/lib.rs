use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

use env_logger;
use log;
use priority_queue::PriorityQueue;
use regex::Regex;
use lazy_static::lazy_static;

use utils;

const FILENAME: &str = "day18\\data.txt";
const X_DIM: usize = 71;
const Y_DIM: usize = 71;
const TAKE_FIRST: usize = 1024;
lazy_static!{
    pub static ref BYTES: Regex = Regex::new(r"(?<x>\d+),(?<y>\d+)").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}


fn get_walls(take_first: usize) -> HashSet<Point> {
    utils::file_to_iter(FILENAME)
        .take(take_first)
        .map(|line| {
            let caps = BYTES.captures(&line).unwrap();
            Point { x: caps["x"].parse::<usize>().unwrap(), y: caps["y"].parse::<usize>().unwrap() }
        })
        .collect::<HashSet<Point>>()
}

fn get_all_walls() -> Vec<Point> {
    utils::file_to_iter(FILENAME)
        .map(|line| {
            let caps = BYTES.captures(&line).unwrap();
            Point { x: caps["x"].parse::<usize>().unwrap(), y: caps["y"].parse::<usize>().unwrap() }
        })
        .collect::<Vec<Point>>()
}

fn render_maze(maze: &HashSet<Point>) {
    for y in 0..Y_DIM {
        for x in 0..X_DIM {
            match maze.contains(&Point { x, y }) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

fn solve_maze(maze: &HashSet<Point>) -> Option<usize> {
    let end = Point { x: X_DIM - 1, y: Y_DIM - 1 };
    let mut pq: PriorityQueue<Point, Reverse<usize>> = PriorityQueue::new();
    let mut min_scores: HashMap<Point, usize> = HashMap::new();
    pq.push_increase(Point{x:0, y:0}, Reverse(1));
    min_scores.insert(Point{x:0, y:0}, 1);
    loop {
        if pq.is_empty() { return None; }
        let (pos, score) = pq.pop().unwrap();
        log::debug!("{:?} {}", pos, score.0);
        if pos == end {
            return Some(score.0 - 1);
        }
        // Left
        if pos.x > 0 {
            let p = Point { x: pos.x - 1, y: pos.y};
            if !maze.contains(&p) && score.0 + 1 < *min_scores.entry(p).or_insert(std::usize::MAX) {
                pq.push_increase(p, Reverse(score.0 + 1));
                min_scores.insert(p, score.0 + 1);
            }
        }
        // Right
        if pos.x < X_DIM - 1{
            let p = Point { x: pos.x + 1, y: pos.y};
            if !maze.contains(&p) && score.0 + 1 < *min_scores.entry(p).or_insert(std::usize::MAX) {
                pq.push_increase(p, Reverse(score.0 + 1));
                min_scores.insert(p, score.0 + 1);
            }
        }
        // Up
        if pos.y > 0 {
            let p = Point { x: pos.x, y: pos.y - 1};
            if !maze.contains(&p) && score.0 + 1 < *min_scores.entry(p).or_insert(std::usize::MAX) {
                pq.push_increase(p, Reverse(score.0 + 1));
                min_scores.insert(p, score.0 + 1);
            }
        }
        // Down
        if pos.y < Y_DIM - 1 {
            let p = Point { x: pos.x, y: pos.y + 1};
            if !maze.contains(&p) && score.0 + 1 < *min_scores.entry(p).or_insert(std::usize::MAX) {
                pq.push_increase(p, Reverse(score.0 + 1));
                min_scores.insert(p, score.0 + 1);
            }
        }
    }
}

pub fn main_day18_task1() {
    _ = env_logger::try_init();

    let maze = get_walls(TAKE_FIRST);
    render_maze(&maze);

    println!("Day 18 task 1 result is {}", solve_maze(&maze).unwrap());
}

pub fn main_day18_task2() {
    _ = env_logger::try_init();

    let walls = get_all_walls();
    let mut maze: HashSet<Point> = HashSet::from_iter(walls.iter().take(TAKE_FIRST).cloned());
    let mut next_wall = Point{x:0, y:0};

    for i in 0.. {
        log::info!("{}", i);
        match solve_maze(&maze) {
            None => {
                println!("Day 18 task 2 result is {},{}", next_wall.x, next_wall.y);
                break
            },
            _ => (),
        }
        next_wall = walls[TAKE_FIRST + i];
        maze.insert(next_wall);
    }
}
