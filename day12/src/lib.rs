use std::collections::{HashMap, HashSet};

use env_logger;
use log;

use utils;

const FILENAME: &str = "day12\\data.txt";

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    // Make it signed so that there are no issues with boundaries
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Plot {
    area: HashSet<Point>,
    boundary: HashSet<Point>,
}

impl Plot {
    fn add(&mut self, point: Point) {
        self.boundary.remove(&point);
        self.area.insert(point.clone());
        self.boundary.insert(Point{x: point.x - 1, y: point.y});
        self.boundary.insert(Point{x: point.x + 1, y: point.y});
        self.boundary.insert(Point{x: point.x, y: point.y - 1});
        self.boundary.insert(Point{x: point.x, y: point.y + 1});
    }

    fn merge(&mut self, other: &Plot) {
        for point in other.area.iter() {
            self.area.insert(point.clone());
        }
        for point in other.boundary.iter() {
            if self.area.contains(&point) { continue }
            self.boundary.insert(point.clone());
        }
    }

    fn calc_area(&self) -> u32 {
        self.area.len() as u32
    }

    fn calc_perimeter(&self) -> u32 {
        let mut total = 0;
        for point in self.area.iter() {
            // Top
            if !self.area.contains(& Point { x: point.x - 1, y: point.y }) { total += 1 }
            // Bottom
            if !self.area.contains(& Point { x: point.x + 1, y: point.y }) { total += 1 }
            // Left
            if !self.area.contains(& Point { x: point.x, y: point.y -1 }) { total += 1 }
            // Right
            if !self.area.contains(& Point { x: point.x, y: point.y + 1 }) { total += 1 }
        }
        total
    }

    fn calc_straight_lines(&self) -> u32 {
        // Calculate straight lines - was not clear from the puzzle
        let mut horizontal = HashSet::new();
        let mut vertical = HashSet::new();
        
        // Get all perimeters
        for point in self.area.iter() {
            // Top
            if !self.area.contains(& Point { x: point.x - 1, y: point.y }) {
                horizontal.insert(Point { x: point.x, y: point.y });
            }
            // Bottom
            if !self.area.contains(& Point { x: point.x + 1, y: point.y }) {
                horizontal.insert(Point { x: point.x + 1, y: point.y });
            }
            // Left
            if !self.area.contains(& Point { x: point.x, y: point.y -1 }) {
                vertical.insert(Point { x: point.x, y: point.y });
            }
            // Right
            if !self.area.contains(& Point { x: point.x, y: point.y + 1 }) {
                vertical.insert(Point { x: point.x, y: point.y + 1 });
            }
        }
        
        let mut total = 0;
        for perimeter in horizontal.iter() {
            if !horizontal.contains(& Point { x: perimeter.x, y: perimeter.y - 1 } ) { total += 1 }
        }
        for perimeter in vertical.iter() {
            if !vertical.contains(& Point { x: perimeter.x - 1, y: perimeter.y } ) { total += 1 }
        }

        total
    }

    fn calc_corners(&self) -> u32 {
        // Calculate corners - was not clear from the puzzle
        let mut inside = 0;
        let mut outside = 0;

        for point in self.area.iter() {
            // Inside
            if self.area.contains(& Point {x: point.x - 1, y: point.y})
                && self.area.contains(& Point {x: point.x, y: point.y + 1})
                && !self.area.contains(& Point {x: point.x - 1, y: point.y + 1})
            { inside += 1; }
            if self.area.contains(& Point {x: point.x - 1, y: point.y})
                && self.area.contains(& Point {x: point.x, y: point.y - 1})
                && !self.area.contains(& Point {x: point.x - 1, y: point.y - 1})
            { inside += 1; }
            if self.area.contains(& Point {x: point.x + 1, y: point.y})
                && self.area.contains(& Point {x: point.x, y: point.y + 1})
                && !self.area.contains(& Point {x: point.x + 1, y: point.y + 1})
            { inside += 1; }
            if self.area.contains(& Point {x: point.x + 1, y: point.y})
                && self.area.contains(& Point {x: point.x, y: point.y - 1})
                && !self.area.contains(& Point {x: point.x + 1, y: point.y - 1})
            { inside += 1; }
            // Outside
            if !self.area.contains(& Point {x: point.x - 1, y: point.y})
                && !self.area.contains(& Point {x: point.x, y: point.y + 1})
            { outside += 1; }
            if !self.area.contains(& Point {x: point.x - 1, y: point.y})
                && !self.area.contains(& Point {x: point.x, y: point.y - 1})
            { outside += 1; }
            if !self.area.contains(& Point {x: point.x + 1, y: point.y})
                && !self.area.contains(& Point {x: point.x, y: point.y - 1})
            { outside += 1; }
            if !self.area.contains(& Point {x: point.x + 1, y: point.y})
                && !self.area.contains(& Point {x: point.x, y: point.y + 1})
            { outside += 1; }
        }
        
        inside + outside
    }
}

fn get_garden() -> HashMap<char, HashMap<String, Plot>> {
    // Use a hashmap of plots because of the merging
    let mut garden: HashMap<char, HashMap<String, Plot>> = HashMap::new();
    
    for (i, line) in utils::file_to_iter(FILENAME).enumerate() {
        for (j, c) in line.chars().enumerate() {
            let id = format!("{i}{j}");
            let new_point = Point{x: i as i32, y: j as i32};
            let mut plot_to_extend = None;
            
            // Add new entry if plant does not exist yet
            let plant = garden.entry(c).or_insert(HashMap::new());
            
            // Search plots if one can be extended
            for (k, plot) in plant.iter() {
                if plot.boundary.contains(&new_point) {
                    plot_to_extend = Some(k.clone());
                    break
                }
            }
            match plot_to_extend {
                None => { // New plot
                    let mut new_plot = Plot{ area: HashSet::new(), boundary: HashSet::new()};
                    new_plot.add(new_point);
                    plant.insert(id, new_plot);
                }
                Some(plot_nr) => { // Extend and merge
                    plant.get_mut(&plot_nr).unwrap().add(new_point);
                    
                    let mut plots_to_merge = vec![];
                    for (k, plot) in plant.iter() {
                        if *k == id { continue }
                        if plot.boundary.contains(&new_point) { plots_to_merge.push(k.clone()) }
                    }
                    let mut plots = vec![];
                    for k in plots_to_merge {
                        plots.push(plant.remove(&k).unwrap());
                    }
                    for plot in plots {
                        plant.get_mut(&plot_nr).unwrap().merge(&plot);
                    }
                },
            }
        }
    }

    garden
}

pub fn main_day12() {
    _ = env_logger::try_init();

    let garden = get_garden();
    for (c, plant) in garden.iter() {
        log::debug!("{:?}", c);
        for plot in plant {
            log::debug!("{:?}", plot.1.area);
        }
    }

    let result: u32 = garden
        .iter()
        .map(|(_, plant)|
            plant
            .iter()
            .map(|(_, plot)| {
                plot.calc_area() * plot.calc_perimeter()
            })
            .sum::<u32>()
        ).sum();
    println!("Day 12 task 1 result is {}", result);

    let result: u32 = garden
        .iter()
        .map(|(c, plant)| {
            log::debug!("{}", c);
            plant
            .iter()
            .map(|(_, plot)| {
                log::debug!("\t{} {}", plot.calc_area(), plot.calc_straight_lines());
                plot.calc_area() * plot.calc_straight_lines()
            })
            .sum::<u32>()
            }).sum();
    println!("Day 12 task 2 result (straight lines) is {}", result);

    let result: u32 = garden
        .iter()
        .map(|(c, plant)| {
            log::debug!("{}", c);
            plant
            .iter()
            .map(|(_, plot)| {
                log::debug!("\tarea={} corners={}", plot.calc_area(), plot.calc_corners());
                plot.calc_area() * plot.calc_corners()
            })
            .sum::<u32>()
            }).sum();
    println!("Day 12 task 2 result (corners) is {}", result);
}
