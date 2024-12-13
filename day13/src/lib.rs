use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use env_logger;
use log;

use utils;

const FILENAME: &str = "day13\\data.txt";
lazy_static!{
    pub static ref REGEX_A: Regex = Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    pub static ref REGEX_B: Regex = Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    pub static ref REGEX_PRIZE: Regex = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq)]
struct Button {
    p: Point,
    cost: i64,
}

fn get_data(shift: i64) -> Vec<(Button, Button, Point)> {
    let mut data = vec![];
    let stream = utils::file_to_iter(FILENAME);

    for lines in &stream.chunks(4) {
        let lines: Vec<String> = lines.collect();
        let caps1 = REGEX_A.captures(&lines[0]).unwrap();
        let caps2 = REGEX_B.captures(&lines[1]).unwrap();
        let caps3 = REGEX_PRIZE.captures(&lines[2]).unwrap();
        
        data.push((
            Button{
            p: Point{
                x: caps1["x"].parse::<i64>().unwrap(),
                y: caps1["y"].parse::<i64>().unwrap(),
            },
            cost: 3,
            },
            Button{
                p: Point{
                    x: caps2["x"].parse::<i64>().unwrap(),
                    y: caps2["y"].parse::<i64>().unwrap(),
                },
                cost: 1,
            },
            Point {
                x: caps3["x"].parse::<i64>().unwrap() + shift,
                y: caps3["y"].parse::<i64>().unwrap() + shift,
            }
        ))
    }

    data
}

fn calculate_cheapest(button1: &Button, button2: &Button, prize: &Point) -> Option<i64> {
    let mut min_cost: Option<i64> = None;

    for i in 0.. {
        let remainder_x = prize.x - i * button1.p.x;
        if remainder_x < 0 { break }
        let remainder_y = prize.y - i * button1.p.y;
        if remainder_y < 0 { break }
        if remainder_x % button2.p.x == 0
            && remainder_y % button2.p.y == 0
            && remainder_x / button2.p.x == remainder_y / button2.p.y {
                let new_cost = i * button1.cost + remainder_x / button2.p.x * button2.cost;
                log::debug!("{} {} {} {}", i, button1.cost, remainder_x / button2.p.x, button2.cost);
                if let Some(value) = min_cost {
                    min_cost = Some(std::cmp::min(value, new_cost))
                } else { min_cost = Some(new_cost) }
        }
    }

    min_cost
}

fn closed_form(button1: &Button, button2: &Button, prize: &Point) -> Option<i64> {
    let numerator = button1.p.x * prize.y - button1.p.y * prize.x;
    let denominator = button1.p.x * button2.p.y - button1.p.y * button2.p.x;
    match numerator % denominator {
        0 => {
            let b = numerator / denominator;
            let a = (prize.x - b * button2.p.x) / button1.p.x;
            Some(a * button1.cost + b * button2.cost)
        },
        _ => None
    }
}

pub fn main_day13_task1() {
    _ = env_logger::try_init();

    let result: i64 = get_data(0)
        .iter()
        .map(|(button1, button2, prize)| calculate_cheapest(button1, button2, prize))
        .flatten()
        .sum();
    println!("Day 13 task 1 result is {}", result);
}

pub fn main_day13_task2() {
    _ = env_logger::try_init();

    let result: i64 = get_data(10000000000000)
        .iter()
        .map(|(button1, button2, prize)| closed_form(button1, button2, prize))
        .flatten()
        .sum();
    println!("Day 13 task 2 result is {}", result);
}
