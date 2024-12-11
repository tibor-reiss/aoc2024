use env_logger;
use log;

use utils;

const FILENAME: &str = "day9\\data.txt";
const RADIX: u32 = 10;

pub fn main_day9_task1() {
    _ = env_logger::try_init();

    let line: Vec<u32> = utils::file_to_iter(FILENAME)
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .collect();

    let mut position = 0;
    let mut is_front_file = true;
    let mut front_file_nr = 0;
    let mut back_file_nr = (line.len() - 1) / 2;
    let mut front_pos = 0;
    let mut back_pos = line.len() -1;
    let mut back_counter = line[back_pos];

    let mut result = 0;

    'outer: loop {
        if is_front_file {
            for i in 0..line[front_pos] {
                log::debug!("{position} front_pos={front_pos} back_pos={back_pos} i={i} back_counter={back_counter}");
                if front_pos == back_pos && i == back_counter {
                    break 'outer
                }
                result += position * front_file_nr;
                position += 1;
            }
            is_front_file = false;
            front_pos += 1;
            front_file_nr += 1;
        }
        else {
            for i in 0..line[front_pos] {
                log::debug!("{position} back_pos={back_pos} front_pos={front_pos} i={i} back_counter={back_counter}");
                result += position * back_file_nr;
                position += 1;
                back_counter -= 1;
                // If the back file size drops to zero, then search for the next file
                if back_counter == 0 {
                    back_pos -= 2;
                    back_counter = line[back_pos];
                    back_file_nr -= 1;
                }
                if front_pos > back_pos {
                    break 'outer
                }
                if front_pos == back_pos && i == back_counter {
                    break 'outer
                }
            }
            is_front_file = true;
            front_pos += 1;
        }
    }

    println!("Day 9 task 1 result is {}", result);
}

pub fn main_day9_task2() {
    _ = env_logger::try_init();

    let line: Vec<u32> = utils::file_to_iter(FILENAME)
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .collect();
    
    // Define how big is the free space, and how much it is already occupied (starts all with 0)
    let mut free_space_map: Vec<(u32, u32)> = line.iter().skip(1).step_by(2).map(|v| (*v, 0)).collect();
    log::debug!("{free_space_map:?}");
    let mut file_system = vec![0; line.iter().sum::<u32>() as usize];
    
    let mut back_file_nr = (line.len() - 1) / 2;
    let mut back_pos = line.len() -1;

    while back_pos > 0 {
        let file_size = line[back_pos];
        let mut file_moved = false;
        
        for (i, (free_space_size, offset)) in free_space_map.iter().enumerate() {
            log::debug!("{file_system:?}");
            
            // If there is a space large enough, move the file
            // But can only move to a place which is before the file
            if *free_space_size >= file_size && back_file_nr > i {
                let where_to_insert = line.iter().take(2 * i + 1).sum::<u32>() + offset;
                let where_to_insert = where_to_insert as usize;
                log::debug!("insert {back_file_nr} at {where_to_insert}");
                for i in where_to_insert..where_to_insert + file_size as usize {
                    file_system[i] = back_file_nr;
                }
                free_space_map[i].0 -= file_size;
                free_space_map[i].1 += file_size;
                file_moved=true;
                break
            }
        }

        // If file was not moved, put it where it was
        if !file_moved {
            let where_to_insert: u32 = line.iter().take(back_pos).sum();
            let where_to_insert = where_to_insert as usize;
            log::debug!("insert {back_file_nr} at {where_to_insert}");
            for i in where_to_insert..where_to_insert + file_size as usize {
                file_system[i] = back_file_nr;
            }
        }
        
        log::debug!("{free_space_map:?}");
        
        back_pos -= 2;
        back_file_nr -= 1;
    }

    log::debug!("{file_system:?}");
    let result: usize = file_system
        .iter()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum();

    println!("Day 9 task 2 result is {}", result);
}
