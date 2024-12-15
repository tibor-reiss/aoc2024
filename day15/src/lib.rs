use std::collections::HashSet;

use env_logger;
use log;

use utils;

const FILENAME: &str = "day15\\data.txt";
const BOX: char = 'O';
const BOX_L: char = '[';
const BOX_R: char = ']';
const WALL: char = '#';
const ROBOT: char = '@';
const EMPTY: char = '.';

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        }
    }

    fn move_point(&self, p: &mut Point) {
        match self {
            Direction::Left => p.y -= 1,
            Direction::Right => p.y += 1,
            Direction::Up => p.x -= 1,
            Direction::Down => p.x += 1,
        }
    }

    fn is_up(&self) -> bool {
        match self {
            Direction::Up => true,
            _ => false,
        }
    }
}

fn move_right(position: &mut Point, boxes: &mut HashSet<Point>, walls: &HashSet<Point>) {
    let mut p = Point { x: position.x, y: position.y + 1 };
    let mut moved_boxes: HashSet<Point> = HashSet::new();
    
    loop {
        if walls.contains(&p) { return }
        // Check the left side of the box
        if !boxes.contains(&p) { break }
        moved_boxes.insert(p.clone());
        p.y += 2;
    }

    for b in moved_boxes.iter() { boxes.remove(&b); }
    for b in moved_boxes.iter() { boxes.insert(Point {x: b.x, y: b.y + 1} ); }
    position.y += 1;
}

fn move_left(position: &mut Point, boxes: &mut HashSet<Point>, walls: &HashSet<Point>)  {
    let mut p = Point { x: position.x, y: position.y - 1 };
    let mut moved_boxes: HashSet<Point> = HashSet::new();
    
    loop {
        if walls.contains(&p) { return }
        // Check the left side of the box
        if !boxes.contains(& Point {x: p.x, y: p.y - 1} ) { break }
        moved_boxes.insert(Point {x: p.x, y: p.y - 1});
        p.y -= 2;
    }

    for b in moved_boxes.iter() { boxes.remove(&b); }
    for b in moved_boxes.iter() { boxes.insert(Point {x: b.x, y: b.y - 1} ); }
    position.y -= 1;
}

fn move_vertical(position: &mut Point, boxes: &mut HashSet<Point>, walls: &HashSet<Point>, direction: Direction) {
    let mut x = if direction.is_up() { position.x - 1 } else { position.x + 1 };
    let mut ys: HashSet<usize> = HashSet::new();
    ys.insert(position.y);
    let mut moved_boxes: HashSet<Point> = HashSet::new();
    
    loop {
        // Found a wall
        if ys.iter().any(|&y| walls.contains(& Point {x, y} )) { return }
        // Found no boxes to push
        if ys.iter().all(|&y| {
            !(
                boxes.contains(& Point {x, y} ) || boxes.contains(& Point {x, y: y - 1} )
            )}
        ) { break }
        // Check where are the boxes and update
        let mut temp_ys: HashSet<usize> = HashSet::new();
        for &y in ys.iter() {
            // @.
            // []
            if boxes.contains(& Point {x, y} ) {
                moved_boxes.insert(Point {x, y});
                temp_ys.insert(y);
                temp_ys.insert(y + 1);
            }
            // .@
            // []
            else if boxes.contains(& Point {x, y: y - 1} ) {
                moved_boxes.insert(Point {x, y: y - 1});
                temp_ys.insert(y - 1);
                temp_ys.insert(y);
            }
        }
        ys = temp_ys.clone();

        if direction.is_up() { x -= 1; } else { x += 1; }
    }
    
    for b in moved_boxes.iter() { boxes.remove(&b); }
    for b in moved_boxes.iter() { boxes.insert(Point {x: if direction.is_up() { b.x - 1 } else { b.x + 1 }, y: b.y} ); }
    if direction.is_up() { position.x -= 1; } else { position.x += 1; }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn get_data() -> (Vec<Vec<char>>, Vec<char>) {
    let mut warehouse = vec![];
    let mut movement = vec![];
    let mut warehouse_done = false;
    
    for line in utils::file_to_iter(FILENAME) {
        if !line.is_empty() && !warehouse_done {
            warehouse.push(line.chars().collect::<Vec<char>>());
            continue
        }
        else if !warehouse_done {
            warehouse_done = true;
            continue
        }

        movement.extend(line.chars());
    }

    (warehouse, movement)
}

fn get_robot(warehouse: &Vec<Vec<char>>) -> Point {
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == ROBOT {
                return Point {x: i, y: j }
            }
        }
    }

    panic!()
}

fn move_boxes(warehouse: &Vec<Vec<char>>, position: &Point, direction: Direction) -> Option<Point> {
    let mut p = Point { x: position.x, y: position.y };

    loop {
        direction.move_point(&mut p);
        match warehouse[p.x][p.y] {
            WALL => { return None; },
            EMPTY => { return Some(p); },
            _ => (),
        }
    }
}

fn gps(warehouse: &Vec<Vec<char>>) -> usize {
    let mut gps = 0;
    
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == BOX { gps += 100 * i + j; }
        }
    }

    gps
}

fn gps2(boxes: &HashSet<Point>) -> usize {
    boxes.iter().map(|b| b.x * 100 + b.y ).sum::<usize>()
}

fn get_boxes(warehouse: &Vec<Vec<char>>) -> HashSet<Point> {
    // Take into account the doubling of the warehouse
    let mut boxes = HashSet::new();
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == BOX { boxes.insert(Point { x: i, y: 2 * j }); }
        }
    }
    log::debug!("{:?}", boxes);
    boxes
}

fn get_walls(warehouse: &Vec<Vec<char>>) -> HashSet<Point> {
    // Take into account the doubling of the warehouse
    let mut walls = HashSet::new();
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == WALL {
                walls.insert(Point { x: i, y: j * 2 });
                walls.insert(Point { x: i, y: j * 2 + 1 });
            }
        }
    }
    log::debug!("{:?}", walls);
    walls
}

fn render_wide_warehouse(dim_x: usize, dim_y: usize, robot: Point, boxes: &HashSet<Point>, walls: &HashSet<Point>) {
    let mut jump = false;

    for i in 0..dim_x {
        let mut to_print = vec!['.'; dim_y];
        for j in 0..dim_y {
            if jump {
                jump = false;
                to_print[j] = BOX_R;
            } else {
                let p = Point { x: i, y: j};
                if p == robot { to_print[j] = ROBOT; }
                else if walls.contains(&p) { to_print[j] = WALL; }
                else if boxes.contains(&p) { to_print[j] = BOX_L; jump = true; }
                else { to_print[j] = EMPTY; }
            }
        }
        log::debug!("{:?}", to_print.iter().collect::<String>());
    }
}

pub fn main_day15_task1() {
    _ = env_logger::try_init();

    let (mut warehouse, movement) = get_data();
    let mut robot = get_robot(&warehouse);
    log::debug!("{:?}", robot);

    for c in movement.iter() {
        let direction = Direction::from(*c);
        let outcome = move_boxes(&warehouse, &robot, direction);
        match outcome {
            None => (),
            Some(p) => {
                warehouse[robot.x][robot.y] = '.';
                direction.move_point(&mut robot);
                warehouse[robot.x][robot.y] = ROBOT;
                if p.x != robot.x || p.y != robot.y { warehouse[p.x][p.y] = BOX; }
            }
        }

        for row in warehouse.iter() {
            log::debug!("{:?}", row.iter().collect::<String>());
        }
    }

    let result = gps(&warehouse);
    println!("Day 15 task 1 result is {}", result);
}

pub fn main_day15_task2() {
    _ = env_logger::try_init();

    let (warehouse, movement) = get_data();
    let mut robot = get_robot(&warehouse);
    robot.y *= 2;
    let mut boxes = get_boxes(&warehouse);
    let walls = get_walls(&warehouse);
    log::debug!("{:?}", robot);

    render_wide_warehouse(warehouse.len(), warehouse[0].len() * 2, robot, &boxes, &walls);
    for c in movement.iter() {
        match Direction::from(*c) {
            Direction::Left => move_left(&mut robot, &mut boxes, &walls),
            Direction::Right => move_right(&mut robot, &mut boxes, &walls),
            d => move_vertical(&mut robot, &mut boxes, &walls, d),
        }
        render_wide_warehouse(warehouse.len(), warehouse[0].len() * 2, robot, &boxes, &walls);
    }

    let result = gps2(&boxes);
    println!("Day 15 task 2 result is {}", result);
}
