use std::collections::HashMap;
use std::io::stdin;

use env_logger;
use lazy_static::lazy_static;
use log;
use regex::Regex;

use utils;

const FILENAME: &str = "day14\\data.txt";
lazy_static!{
    //p=0,4 v=3,-3
    pub static ref RE: Regex = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn move_robot(robot: &Robot, t: i64, dim_x: i64, dim_y: i64) -> Robot {
    Robot {
        x: (robot.x + robot.vx * t).rem_euclid(dim_x),
        y: (robot.y + robot.vy * t).rem_euclid(dim_y),
        vx: robot.vx,
        vy: robot.vy,
    }
}

fn get_quadrant(robot: &Robot, dim_x: i64, dim_y: i64) -> i64 {
    if robot.x < dim_x / 2 && robot.y < dim_y / 2 { return 1 }
    if robot.x < dim_x / 2 && robot.y > dim_y / 2 { return 3 }
    if robot.x > dim_x / 2 && robot.y < dim_y / 2 { return 2 }
    if robot.x > dim_x / 2 && robot.y > dim_y / 2 { return 4 }
    0
}

fn get_data() -> Vec<Robot> {
    utils::file_to_iter(FILENAME).map(|s| {
        let caps = RE.captures(&s).unwrap();
        Robot{
            x: caps["x"].parse::<i64>().unwrap(),
            y: caps["y"].parse::<i64>().unwrap(),
            vx: caps["vx"].parse::<i64>().unwrap(),
            vy: caps["vy"].parse::<i64>().unwrap(),
        }
    }).collect::<Vec<Robot>>()
}

pub fn main_day14_task1() {
    _ = env_logger::try_init();

    let robots = get_data();

    let dim_x = 101;
    let dim_y = 103;

    let mut quadrants: HashMap<i64, u64> = HashMap::new();
    for robot in robots {
        let robot = move_robot(&robot, 100, dim_x, dim_y);
        let quadrant = get_quadrant(&robot, dim_x, dim_y);
        *quadrants.entry(quadrant).or_insert(0) += 1;
        log::debug!("{} {}", robot.x, robot.y);
    }
    
    log::debug!("{}", quadrants.get(&1).unwrap_or(&0));
    log::debug!("{}", quadrants.get(&2).unwrap_or(&0));
    log::debug!("{}", quadrants.get(&3).unwrap_or(&0));
    log::debug!("{}", quadrants.get(&4).unwrap_or(&0));
    let result = quadrants.get(&1).unwrap_or(&0)
        * quadrants.get(&2).unwrap_or(&0)
        * quadrants.get(&3).unwrap_or(&0)
        * quadrants.get(&4).unwrap_or(&0);
    println!("Day 14 task 1 result is {}", result);
}

pub fn main_day14_task2() {
    _ = env_logger::try_init();

    let robots = get_data();

    let dim_x = 101;
    let dim_y = 103;

    for i in 0..10000000 {
        let mut state = vec![vec![0; dim_x]; dim_y];
        for robot in robots.iter() {
            let new_robot = move_robot(&robot, i, dim_x as i64, dim_y as i64);
            state[new_robot.y as usize][new_robot.x as usize] += 1;
        }

        let mut total_x = 0;
        let mut total_y = 0;
        for (u, y) in state.iter().enumerate() {
            for (v, x) in y.iter().enumerate() {
                if v > 40 && v < 60 && *x > 0 { total_x += 1; }
                if u > 40 && u < 60 && *x > 0 { total_y += 1; }
            }
        }

        if total_x > 200 && total_y > 150 {
            println!("##############################{i}##############################");
            for y in state.iter() {
                for x in y.iter() {
                    if *x > 0 { print!("*"); }
                    else { print!(" "); }
                }
                println!();
            }
            let mut s=String::new();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
        }
    }
}
