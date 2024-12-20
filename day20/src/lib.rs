use std::collections::{HashMap, HashSet};

use env_logger;
use log;
use itertools::Itertools;

use utils;

const FILENAME: &str = "day20\\data.txt";

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    // It can't be negative, but still use this because it will be easier when counting the cheats
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as i32
    }
}

fn get_maze() -> (HashSet<Point>, Point, Point, i32, i32) {
    let mut start: Point = Point{x:0, y:0};
    let mut end: Point = Point{x:0, y:0};
    let mut walls = HashSet::new();
    let y_dim = utils::file_to_string_vector(FILENAME).len() as i32;
    let x_dim = utils::file_to_string_vector(FILENAME)[0].len() as i32;

    for (i, line) in utils::file_to_iter(FILENAME).enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => { start = Point { x: j as i32, y: i as i32 }; },
                'E' => { end = Point { x: j as i32 , y: i as i32 }; },
                '#' => { walls.insert(Point {x: j as i32, y: i as i32} ); },
                _ => (),
            }
        }
    }

    (walls, start, end, x_dim, y_dim)
}

fn render_maze(walls: &HashSet<Point>, x_dim: i32, y_dim: i32, start: &Point, end: &Point) {
    for y in 0..y_dim {
        for x in 0..x_dim {
            let p = Point { x: x as i32, y: y as i32 };
            if p == *start { print!("S"); continue }
            if p == *end { print!("E"); continue }
            match walls.contains(&p) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

fn get_solution(start: &Point, end: &Point, walls: &HashSet<Point>) -> HashMap<Point, i32> {
    // The solution is a HashMap between the point and it's position in the solution
    let mut solution: HashMap<Point, i32> = HashMap::new();
    let mut p = start.clone();
    let mut p2 = start.clone();
    let mut counter = 0;
    solution.insert(p, counter);
    
    loop {
        p = p2.clone();
        if p == *end { break }
        counter += 1;
        
        // Up
        p2 = Point { x: p.x, y: p.y - 1 };
        if !walls.contains(&p2) && !solution.contains_key(&p2) { solution.insert(p2, counter); continue }
        // Down
        p2 = Point { x: p.x, y: p.y + 1 };
        if !walls.contains(&p2) && !solution.contains_key(&p2) { solution.insert(p2, counter); continue }
        // Left
        p2 = Point { x: p.x - 1, y: p.y };
        if !walls.contains(&p2) && !solution.contains_key(&p2) { solution.insert(p2, counter); continue }
        // Right
        p2 = Point { x: p.x + 1, y: p.y };
        if !walls.contains(&p2) && !solution.contains_key(&p2) { solution.insert(p2, counter); continue }

    }

    solution
}

fn get_cheats(walls: &HashSet<Point>, solution: &HashMap<Point, i32>, x_dim: i32, y_dim: i32) -> HashMap<i32, i32> {
    // Count the cheats in buckets
    let mut cheats: HashMap<i32, i32> = HashMap::new();
    let mut p1: Point;
    let mut p2: Point;

    for (p, pos) in solution {
        // Up
        if p.y > 1 {
            p1 = Point{x: p.x, y: p.y - 1};
            p2 = Point{x: p.x, y: p.y - 2};
            if walls.contains(&p1) {
                if let Some(v) = solution.get(&p2) {
                    if v > pos {
                        *cheats.entry(v-pos-2).or_insert(0) += 1;
                    }
                }
            }
        }
        // Down
        if p.y < y_dim - 1 {
            p1 = Point{x: p.x, y: p.y + 1};
            p2 = Point{x: p.x, y: p.y + 2};
            if walls.contains(&p1) {
                if let Some(v) = solution.get(&p2) {
                    if v > pos {
                        *cheats.entry(v-pos-2).or_insert(0) += 1;
                    }
                }
            }
        }
        // Left
        if p.x > 1 {
            p1 = Point{x: p.x - 1, y: p.y};
            p2 = Point{x: p.x - 2, y: p.y};
            if walls.contains(&p1) {
                if let Some(v) = solution.get(&p2) {
                    if v > pos {
                        *cheats.entry(v-pos-2).or_insert(0) += 1;
                    }
                }
            }
        }
        // Right
        if p.x < x_dim - 1 {
            p1 = Point{x: p.x + 1, y: p.y};
            p2 = Point{x: p.x + 2, y: p.y};
            if walls.contains(&p1) {
                if let Some(v) = solution.get(&p2) {
                    if v > pos {
                        *cheats.entry(v-pos-2).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    cheats
}

pub fn main_day20_task1() {
    _ = env_logger::try_init();

    let (walls, start, end, x_dim, y_dim) = get_maze();
    render_maze(&walls, x_dim, y_dim, &start, &end);

    let solution = get_solution(&start, &end, &walls);
    log::info!("Solution length = {}, fastest time = {}", solution.len(), solution.len() - 1);
    
    let cheats = get_cheats(&walls, &solution, x_dim, y_dim);
    for cheat in cheats.keys().sorted() {
        log::debug!("{} {}", cheat, cheats[cheat]);
    }

    println!("Day 20 task 1 result is {}", cheats
        .iter()
        .map(|cheat| {
            match *cheat.0 >= 100 {
                true => *cheat.1,
                false => 0,
            }
        })
        .sum::<i32>()
    );
}

pub fn main_day20_task2() {
    _ = env_logger::builder().format_timestamp(None).try_init();

    let (walls, start, end, x_dim, y_dim) = get_maze();
    let solution = get_solution(&start, &end, &walls);
    log::info!("Solution length = {}, fastest time = {}", solution.len(), solution.len() - 1);

    let limit = 20;
    let min_cheat_amount = 100;

    let mut cheats = HashMap::new();
    for (p, pos) in solution.iter() {
        for x in -limit..=limit {
            let x_new = p.x + x;
            if x_new < 0 || x_new >= x_dim { continue }
            for y in -limit..=limit {
                let y_new = p.y + y;
                if y_new < 0 || y_new >= y_dim { continue }
                let p2 = Point { x: x_new, y: y_new };
                let dist = p.distance(&p2);
                if dist > limit { continue }
                if let Some(&pos2) = solution.get(&p2) {
                    if pos2 - pos - dist >= min_cheat_amount {
                        log::debug!("{} {} {} {} {} {} {}", p.x, p.y, pos, p2.x, p2.y, pos2, dist);
                        *cheats.entry(pos2 - pos - dist).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    println!("Day 20 task 2 result is {}", cheats
        .iter()
        .map(|cheat| {
            match *cheat.0 >= min_cheat_amount {
                true => *cheat.1,
                false => 0,
            }
        })
        .sum::<i32>()
    );
}
