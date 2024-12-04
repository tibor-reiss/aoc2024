use utils;

fn get_numbers(line: &str) -> Vec<i64> {
    line
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn is_block_safe(numbers: &[i64]) -> bool {
    let mut is_increasing = true;
    
    match numbers.len() {
        0 | 1 => return true,
        _ => {},
    }
    
    let mut prev_number = numbers[0];
    if numbers[1] < prev_number {
        is_increasing = false;
    }
    for &number in numbers.iter().skip(1) {
        if (number - prev_number).abs() > 3 { return false }
        if is_increasing && number <= prev_number { return false }
        if !is_increasing && number >= prev_number { return false }
        prev_number = number;
    }

    true
}

fn is_line_error_free(line: &str) -> bool {
    let numbers = get_numbers(line);
    is_block_safe(&numbers)
}

fn is_line_with_max_one_error(line: &str) -> bool {
    let numbers = get_numbers(line);
    
    //Check whole sequence
    if is_block_safe(&numbers) { return true }
    
    //Check all possible slices leaving 1 element out
    for i in 0..numbers.len() {
        let mut leave_one = numbers.iter().take(i).collect::<Vec<_>>();
        leave_one.extend(numbers.iter().skip(i+1));
        if is_block_safe(&leave_one.into_iter().cloned().collect::<Vec<_>>()) { return true }
    }

    false
}

pub fn main_day2_task1() {
    let lines = utils::file_to_string_vector("day2\\data.txt");
    let result = lines.iter().filter(|line| is_line_error_free(line)).count();
    println!("Day 2 task 1 result is {}", result);
}

pub fn main_day2_task2() {
    let lines = utils::file_to_string_vector("day2\\data.txt");
    let result = lines.iter().filter(|line| is_line_with_max_one_error(line)).count();
    println!("Day 2 task 2 result is {}", result);
}
