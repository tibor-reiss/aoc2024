use utils;

fn move_dir(pos: usize, increase: Option<bool>, distance: usize, limit: usize) -> Option<usize> {
    //Helper function to stay in usize all the way
    match increase {
        None => Some(pos),
        Some(true) => if pos + distance >= limit { None } else { Some(pos + distance) },
        Some(false) => if distance > pos { None } else { Some(pos - distance) },
    }
}

fn check_word(
    letters: &Vec<Vec<char>>,
    x_dim: usize,
    y_dim: usize,
    x_pos: usize,
    y_pos: usize,
    x_is_increase: Option<bool>,
    x_distance: usize,
    y_is_increase: Option<bool>,
    y_distance: usize,
    remaining_letters: &[char],
) -> u64 {
    if letters[x_pos][y_pos] != remaining_letters[0 as usize] { return 0 }
    // If it was the last letter, found 1
    if remaining_letters.len() == 1 { return 1 }
    let new_x_pos = move_dir(x_pos, x_is_increase, x_distance, x_dim);
    let new_y_pos = move_dir(y_pos, y_is_increase, y_distance, y_dim);
    if new_x_pos.is_none() || new_y_pos.is_none() { return 0 }
    let new_x_pos = new_x_pos.unwrap();
    let new_y_pos = new_y_pos.unwrap();
    return check_word(
        letters,
        x_dim, y_dim,
        new_x_pos, new_y_pos,
        x_is_increase, x_distance,
        y_is_increase, y_distance,
        &remaining_letters[1..],
    )
}

fn get_letters() -> Vec<Vec<char>> {
    //Read into a 2d array
    let mut letters = vec![];
    for (i, line) in utils::file_to_iter("day4\\data.txt").enumerate() {
        letters.push(vec![]);
        for char in line.chars() {
            letters[i].push(char);
        }
    }
    letters
}

pub fn main_day4_task1() {
    let letters = get_letters();
    let x_dim = letters.len();
    let y_dim = letters[0].len();

    let remaining_letters = ['X', 'M', 'A', 'S'];
    let mut result = 0;
    //By checking whether the first letter matches one could trade some speed against generality
    for i in 0..x_dim {
        for j in 0..y_dim {
            //Check all directions
            result +=
                check_word(&letters, x_dim, y_dim, i, j, Some(false), 1, Some(false), 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, Some(false), 1, None, 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, Some(false), 1, Some(true), 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, None, 1, Some(false), 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, None, 1, Some(true), 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, Some(true), 1, Some(false), 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, Some(true), 1, None, 1, &remaining_letters)
                + check_word(&letters, x_dim, y_dim, i, j, Some(true), 1, Some(true), 1, &remaining_letters);
        }
    }
    println!("Day 4 task 1 result is {}", result);
}

pub fn main_day4_task2() {
    let letters = get_letters();
    let mut result = 0;

    for i in 0..letters.len()-2 {
        for j in 0..letters.len()-2 {
            if letters[i+1][j+1] != 'A' { continue }
            if (letters[i][j] == 'M' && letters[i+2][j] == 'M' && letters[i][j+2] == 'S' && letters[i+2][j+2] == 'S')
                || (letters[i][j] == 'M' && letters[i+2][j] == 'S' && letters[i][j+2] == 'M' && letters[i+2][j+2] == 'S')
                || (letters[i][j] == 'S' && letters[i+2][j] == 'S' && letters[i][j+2] == 'M' && letters[i+2][j+2] == 'M')
                || (letters[i][j] == 'S' && letters[i+2][j] == 'M' && letters[i][j+2] == 'S' && letters[i+2][j+2] == 'M')
            { result += 1 }
        }
    }
    
    println!("Day 4 task 2 result is {}", result);
}
