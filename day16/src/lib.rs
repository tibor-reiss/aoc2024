use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

use env_logger;
use log;
use priority_queue::PriorityQueue;

use utils;

const FILENAME: &str = "day16\\data.txt";
const WALL: char = '#';
const END: char = 'E';
const START: char = 'S';

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug,Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }
    
    fn distance(&self, other: &Direction) -> u32 {
        match self {
            Direction::N => {
                match other {
                    Direction::N => 0,
                    Direction::S => 2000,
                    _ => 1000,
                }
            },
            Direction::S => {
                match other {
                    Direction::S => 0,
                    Direction::N => 2000,
                    _ => 1000,
                }
            },
            Direction::W => {
                match other {
                    Direction::W => 0,
                    Direction::E => 2000,
                    _ => 1000,
                }
            },
            Direction::E => {
                match other {
                    Direction::E => 0,
                    Direction::W => 2000,
                    _ => 1000,
                }
            },
        }
    }
}

#[derive(Debug,Eq, PartialEq, Hash)]
struct Step {
    p: Point,
    d: Direction,
}

#[derive(Debug,Eq, PartialEq, Hash)]
struct StepWithPath {
    p: Point,
    d: Direction,
    path: String,
}

fn get_maze() -> (Point, Point, HashSet<Point>) {
    // x from top to bottom (N->S)
    // y from left to right (W->E)
    let mut walls: HashSet<Point> = HashSet::new();
    let mut start = Point { x: 0, y: 0};
    let mut end = Point { x: 0, y: 0};
    
    for (i, line) in utils::file_to_iter(FILENAME).enumerate() {
        for (j, c) in line.chars().enumerate() {
            let p = Point { x: i, y: j};
            match c {
                WALL => { walls.insert(p); },
                END => { end = p; },
                START => { start = p; },
                _ => (),
            }
        }
    }

    (start, end, walls)
}

fn extend_paths(paths: &mut HashMap<Point, HashSet<String>>, prev_point: &Point, next_point: &Point, extension: &str) {
    // Take all the available strings at position "prev_point", extend them with "extension", and add to "next_point"
    // This keeps track of all best/shortest paths
    
    let mut new_paths = HashSet::new();

    paths.entry(*next_point).or_insert(HashSet::new());
    for path in paths.get(&prev_point).unwrap().iter() {
        new_paths.insert(format!("{path}{extension}"));
    }
    paths.get_mut(&next_point).unwrap().extend(new_paths);
}

fn step_one(step: &Step, score: u32, direction: Direction, walls: &HashSet<Point>, minimum_score: u32, min_scores: &mut HashMap<Point, u32>, paths: &mut HashMap<Point, HashSet<String>>, pq: &mut PriorityQueue<Step, Reverse<u32>>) {
    // Temporary variables for better readability
    let ts = score + 1 + step.d.distance(&direction);
    let (tp, extension) = match direction {
        Direction::N => (
            Point { x: step.p.x - 1, y: step.p.y },
            "N",
        ),
        Direction::S => (
            Point { x: step.p.x + 1, y: step.p.y },
            "S",
        ),
        Direction::W => (
            Point { x: step.p.x, y: step.p.y - 1 },
            "W",
        ),
        Direction::E => (
            Point { x: step.p.x, y: step.p.y + 1},
            "E",
        ),
    };

    // Do not step in opposite direction
    // Do not step into a wall
    // Do not check if over the minimum score
    if step.d != direction.opposite() && !walls.contains(&tp) && ts <= minimum_score {
        let cs = min_scores.entry(tp).or_insert(ts);
        // Either the new score (ts) is <= than the current score (cs)
        // Or the special case: there is exactly one turn which can happen e.g. in the following case
        // #####
        // ###E#
        // #..A#
        // #.#.#
        // #...#
        // #S###
        // #####
        // Here at position A, coming from the left one would have a score of 2005, and a score of 3005 coming
        // from the bottom. But coming from the left there is one more turn needed to reach E.
        if ts <= *cs || ts == *cs + 1000 {
            if ts < *cs { *cs = ts; }
            pq.push_increase(
                Step {p: tp, d: direction.clone() },
                Reverse(ts),
            );
            extend_paths(paths, &step.p, &tp, extension);
        }
    }
}

pub fn main_day16_task1() {
    _ = env_logger::try_init();

    let (start, end, walls) = get_maze();
    log::debug!("{:?}", start);
    log::debug!("{:?}", end);
    // A reverse priority queue because we want the lowest score
    let mut pq: PriorityQueue<Step, Reverse<u32>> = PriorityQueue::new();
    let mut minimum_score = std::u32::MAX;
    let mut tp: Point = Point { x: 0, y: 0};

    pq.push_increase(Step { p: start, d: Direction::E }, Reverse(0));
    loop {
        let (step, score) = pq.pop().unwrap();
        
        // Stop if there are only paths left greater than the minimum_score
        if score.0 > minimum_score { break }

        if step.p == end {
            minimum_score = score.0;
        }
        
        // Move N
        tp.x = step.p.x - 1; tp.y = step.p.y;
        if step.d != Direction::S && !walls.contains(&tp) {
            pq.push_increase(
                Step {p: tp, d: Direction::N },
                Reverse(score.0 + 1 + step.d.distance(&Direction::N))
            );
        }
        // Move S
        tp.x = step.p.x + 1; tp.y = step.p.y;
        if step.d != Direction::N && !walls.contains(&tp) {
            pq.push_increase(
                Step { p: tp, d: Direction::S },
                Reverse(score.0 + 1 + step.d.distance(&Direction::S))
            );
        }
        // Move W
        tp.x = step.p.x; tp.y = step.p.y - 1;
        if step.d != Direction::E && !walls.contains(&tp) {
            pq.push_increase(
                Step { p: tp, d: Direction::W },
                Reverse(score.0 + 1 + step.d.distance(&Direction::W))
            );
        }
        // Move E
        tp.x = step.p.x; tp.y = step.p.y + 1;
        if step.d != Direction::W && !walls.contains(&tp) {
            pq.push_increase(
                Step { p: tp, d: Direction::E },
                Reverse(score.0 + 1 + step.d.distance(&Direction::E))
            );
        }
    };

    println!("Day 16 task 1 result is {}", minimum_score);
}

pub fn main_day16_task2() {
    _ = env_logger::try_init();

    let (start, end, walls) = get_maze();
    log::debug!("{:?}", start);
    log::debug!("{:?}", end);
    // A reverse priority queue because we want the lowest score
    let mut pq: PriorityQueue<Step, Reverse<u32>> = PriorityQueue::new();
    // Global minimum_score
    let mut minimum_score = std::u32::MAX;
    // Keep track of the minimum_score at each position
    let mut min_scores: HashMap<Point, u32> = HashMap::new();
    // Keep track of all the best/shortest paths
    let mut paths: HashMap<Point, HashSet<String>> = HashMap::new();

    // Start at "start", in direction "E"
    pq.push_increase(Step { p: start, d: Direction::E }, Reverse(0));
    paths.insert(start, HashSet::new());
    paths.get_mut(&start).unwrap().insert(String::from(""));
    min_scores.insert(start, 0);

    loop {
        if pq.is_empty() { break }
        let (step, score) = pq.pop().unwrap();
        log::debug!("{:?} score={} len={}", step, score.0, pq.len());

        // Stop if there are only paths left greater than the minimum_score
        if score.0 > minimum_score { break }

        if step.p == end {
            minimum_score = score.0;
            continue
        }
        
        // Move N
        step_one(&step, score.0, Direction::N, &walls, minimum_score, &mut min_scores, &mut paths, &mut pq);

        // Move S
        step_one(&step, score.0, Direction::S, &walls, minimum_score, &mut min_scores, &mut paths, &mut pq);
        
        // Move W
        step_one(&step, score.0, Direction::W, &walls, minimum_score, &mut min_scores, &mut paths, &mut pq);

        // Move E
        step_one(&step, score.0, Direction::E, &walls, minimum_score, &mut min_scores, &mut paths, &mut pq);
    };

    let mut tiles: HashSet<Point> = HashSet::new();
    tiles.insert(start);
    let best_paths = paths.get(&end).unwrap();
    log::debug!("Number of best paths = {}", best_paths.len());
    for path in best_paths {
        log::debug!("{path}");
        let mut x = start.x;
        let mut y = start.y;
        for c in path.chars() {
            match c {
                'N' => { x -= 1; },
                'S' => { x += 1; },
                'W' => { y -= 1; },
                'E' => { y += 1;},
                _ => (),
            }
            tiles.insert(Point {x, y} );
        }
    }

    println!("Day 16 task 1 result is {}", minimum_score);
    println!("Day 16 task 2 result is {}", tiles.len());
}
