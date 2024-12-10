use std::collections::HashSet;

use env_logger;
use log;

use utils;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Position {
    x: usize,
    y: usize,
    direction: Direction,
    x_limit: usize,
    y_limit: usize,
}

impl Position {
    fn out_of_bounds_on_next(&self) -> bool {
        if (self.x == 0 && self.direction == Direction::Up)
            || (self.x == self.x_limit - 1 && self.direction == Direction::Down)
            || (self.y == 0 && self.direction == Direction::Left)
            || (self.y == self.y_limit - 1 && self.direction == Direction::Right)
        { true }
        else { false }
    }
    
    fn move_next(&mut self, maze: &Vec<Vec<u8>>, visited: &mut Vec<Vec<Vec<Direction>>>) -> bool {
        if self.out_of_bounds_on_next() { return false; }
        match self.direction {
            Direction::Up => {
                log::debug!("Up {} {} {}", self.x-1, self.y-1, maze[self.x-1][self.y]);
                match maze[self.x-1][self.y] {
                    0 => { self.x -= 1; },
                    _ => { self.direction = self.direction.turn_right(); },
                }
            },
            Direction::Down => {
                log::debug!("Down {} {} {}", self.x+1, self.y, maze[self.x+1][self.y]);
                match maze[self.x+1][self.y] {
                    0 => { self.x += 1; },
                    _ => { self.direction = self.direction.turn_right(); },
                }
            },
            Direction::Right => {
                log::debug!("Right {} {} {}", self.x, self.y+1, maze[self.x][self.y+1]);
                match maze[self.x][self.y+1] {
                    0 => { self.y += 1; },
                    _ => { self.direction = self.direction.turn_right(); },
                }
            },
            Direction::Left => {
                log::debug!("Left {} {} {}", self.x, self.y-1, maze[self.x][self.y-1]);
                match maze[self.x][self.y-1] {
                    0 => { self.y -= 1; }
                    _ => { self.direction = self.direction.turn_right(); },
                }
            },
        }
        if visited[self.x][self.y].contains(&self.direction) { return false; }
        else { visited[self.x][self.y].push(self.direction) }

        true
    }
}

fn get_maze() -> (usize, usize, Vec<Vec<u8>>) {
    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let maze: Vec<Vec<u8>> = utils::file_to_iter("day6\\data.txt")
        .enumerate()
        .map(|(i, line)| line
            .chars()
            .enumerate()
            .map(|(j, c)| {
                match c {
                    '.' => 0,
                    '#' => 1,
                    _ => {
                        start_x = i;
                        start_y = j;
                        0
                    },
                }
            })
            .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>();
    
    (start_x, start_y, maze)
}

pub fn main_day6_task1() {
    let _ = env_logger::try_init();

    let (start_x, start_y, maze) = get_maze();
    
    log::debug!("Start: {} {}", start_x, start_y);
    for line in &maze {
        log::debug!("{:?}", line);
    }

    let mut visited: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; maze[0].len()]; maze.len()];
    visited[start_x][start_y].push(Direction::Up);
    let mut pos = Position{ x: start_x, y: start_y, direction: Direction::Up, x_limit: maze.len(), y_limit: maze[0].len() };

    while pos.move_next(&maze, &mut visited) {
        log::debug!("Position = {} {}", pos.x, pos.y);
    }

    let result: u64 = visited
        .iter()
        .map(|line| line
            .iter()
            .map(|directions| if directions.is_empty() { 0 } else { 1 })
            .sum::<u64>()
        ).sum();
    println!("Day 6 task 1 result is {:?}", result);
}

pub fn main_day6_task2() {
    let _ = env_logger::try_init();

    let (start_x, start_y, maze) = get_maze();
    let mut visited: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; maze[0].len()]; maze.len()];
    let mut pos = Position{ x: start_x, y: start_y, direction: Direction::Up, x_limit: maze.len(), y_limit: maze[0].len() };
    let mut loops: HashSet<(usize, usize)> = HashSet::new();
    while pos.move_next(&maze, &mut visited) {
        let mut temp_maze = maze.clone();
        temp_maze[pos.x][pos.y] = 1;
        for line in &temp_maze {
            log::debug!("{:?}", line);
        }
        let mut temp_visited: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; maze[0].len()]; maze.len()];
        let mut temp_pos = Position{ x: start_x, y: start_y, direction: Direction::Up, x_limit: maze.len(), y_limit: maze[0].len() };
        while temp_pos.move_next(&temp_maze, &mut temp_visited) {
            log::debug!("Position = {} {}", temp_pos.x, temp_pos.y);
        }
        if !temp_pos.out_of_bounds_on_next() {
            log::info!("Loop found for {} {}", pos.x, pos.y);
            loops.insert((pos.x, pos.y));
        }
    }
    log::info!("{:?}", loops);
    println!("Day 6 task 2 result is {:?}", loops.len());
}
