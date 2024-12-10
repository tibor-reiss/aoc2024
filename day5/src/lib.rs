use std::collections::{HashMap, HashSet};

use utils;

const FILENAME: &str = "day5\\data.txt";

fn get_rules() -> HashMap<u64, HashSet<u64>> {
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();

    for line in utils::file_to_iter(FILENAME) {
        if line.contains('|') {
            let values = line
                .split('|')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            rules
                .entry(values[0])
                .or_insert(HashSet::new())
                .insert(values[1]);
        }
    }

    rules
}

fn get_pages() -> Vec<Vec<u64>> {
    let mut pages: Vec<Vec<u64>> = vec![];

    for line in utils::file_to_iter(FILENAME) {
        if line.contains(',') {
            pages.push(line.split(',').map(|v| v.parse::<u64>().unwrap()).collect());
        }
    }

    pages
}

fn order_page(rules: &HashMap<u64, HashSet<u64>>, page: &Vec<u64>) -> Vec<u64> {
    let mut ordered_page = vec![];

    for value_to_insert in page {
        let depends_on = rules.get(value_to_insert);

        match depends_on {
            None => {
                ordered_page.push(*value_to_insert);
            },
            Some(depends_on) => {
                let mut value_inserted = false;
                for (i, value) in ordered_page.iter().enumerate() {
                    if depends_on.contains(value) {
                        ordered_page.insert(i, *value_to_insert);
                        value_inserted = true;
                        break
                    }
                }
                if !value_inserted { ordered_page.push(*value_to_insert) }
            }
        }
    }

    ordered_page
}

pub fn main_day5_task1() -> Vec<Vec<u64>> {
    let rules = get_rules();
    let pages = get_pages();
    let mut result = 0;
    let mut is_wrong: bool;
    let mut wrong_pages = vec![];
    //Go through every page. If the intersection between the rules for this page and the numbers before is not empty, it is incorrectly ordered.
    for page in pages {
        is_wrong = false;
        for (i, value) in page.iter().enumerate() {
            if let Some(set1) = rules.get(value) {
                let set2 = HashSet::<u64>::from_iter(page[..i].iter().cloned());
                if let Some(_) = set2.intersection(&set1).next() {
                    is_wrong = true;
                    break;
                }
            }
        }
        if is_wrong {
            wrong_pages.push(page);
        }
        else {
            result += page[(page.len() - 1) / 2];
        }
    }

    println!("Day 5 task 1 result is {}", result);

    wrong_pages
}

pub fn main_day5_task2(wrong_pages: &Vec<Vec<u64>>) {
    let rules = get_rules();
    let result: u64 = wrong_pages
        .iter()
        .map(|page| order_page(&rules, page)[(page.len() - 1) / 2])
        .sum();

    println!("Day 5 task 2 result is {}", result);
}
