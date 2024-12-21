use std::collections::HashMap;

use env_logger;
use log;

use utils;

const FILENAME: &str = "day21\\data_sample.txt";

fn get_next(c1: char, c2: char) -> &'static str {
    match c1 {
        'A' => match c2 {
            'A' => "A",
            '>' => "vA",
            '<' => "v<<A",
            '^' => "<A",
            'v' => "<vA",
            _ => "",
        }
        '>' => match c2 {
            'A' => "^A",
            '>' => "A",
            '<' => "<<A",
            '^' => "<^A",
            'v' => "<A",
            _ => "",
        }
        '<' => match c2 {
            'A' => ">>^A",
            '>' => ">>A",
            '<' => "A",
            '^' => ">^A",
            'v' => ">A",
            _ => "",
        }
        '^' => match c2 {
            'A' => ">A",
            '>' => "v>A",
            '<' => "v<A",
            '^' => "A",
            'v' => "vA",
            _ => "",
        }
        'v' => match c2 {
            'A' => "^>A",
            '>' => ">A",
            '<' => "<A",
            '^' => "^A",
            'v' => "A",
            _ => "",
        }
        _ => "",
    }
}

fn solve(s: &str) -> String {
    let mut c1 = 'A';
    let mut s2: String = String::from("");
    for c2 in s.chars() {
        s2 += get_next(c1, c2);
        c1 = c2;
    }
    s2
}

pub fn main_day21_task1() {
    _ = env_logger::builder().format_timestamp(None).try_init();
    
    let v = vec![
        ("^^^<AvvvA^^Avv>A", 805), // 805A -> 72
        ("^<<A^^A>vvvA>A", 170), // 170A -> 64
        ("^<<A>A^^>AvvvA", 129), // 129A -> 70
        ("^<A^^Avv>AvA", 283), // 283A -> 68
        ("^^<A<A>vvA>A", 540), // 540A -> 68
    ];
    let v1 = vec![
        ("^^^<AvvvA^^Avv>A", 805), // 805A
        ("^^^<AvvvA^^A>vvA", 805), // 805A
        ("<^^^AvvvA^^Avv>A", 805), // 805A
        ("<^^^AvvvA^^A>vvA", 805), // 805A
    ];
    let v2 = vec![
        ("^<<A^^A>vvvA>A", 170), // 170A
    ];
    let v3 = vec![
        ("^<<A>A>^^AvvvA", 129), // 129A
        ("^<<A>A^^>AvvvA", 129), // 129A
    ];
    let v4 = vec![
        ("^<A^^Avv>AvA", 283), // 283A
        ("^<A^^A>vvAvA", 283), // 283A
        ("<^A^^Avv>AvA", 283), // 283A
        ("<^A^^A>vvAvA", 283), // 283A
    ];
    let v5 = vec![
        ("^^<A<A>vvA>A", 540), // 540A
        ("<^^A<A>vvA>A", 540), // 540A
    ];

    let mut v_all = vec![v1, v2, v3, v4, v5];
    for v in v_all {
        for (s, nr) in v.iter() {
            log::debug!("{} {nr}", solve(&solve(s)).len());
        }
    }

    /*
    let v2 = vec![
        ("<A^A>^^AvvvA", 29), // 029A -> 68 * 29
        ("^^^A<AvvvA>A", 980), // 980A -> 60 * 980
        ("^<<A^^A>>AvvvA", 179), // 179A -> 68 * 179
        ("^^<<A>A>AvvA", 456), // 456A -> 64 * 456
        ("^A<<^^A>>AvvvA", 379), // 379A -> 64 * 379
    ];
    let mut total = 0;
    for (s, nr) in v2.iter() {
        log::debug!("{} {nr}", solve(&solve(s)).len());
        total += solve(&solve(s)).len() * nr;
    }
    */

    /*
    let p1 = "<A^A>^^AvvvA";
    println!("<A^A^^>AvvvA");
    println!("{p1}");

    let p2 = solve(p1);
    println!("v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    println!("{p2}");

    let p3 = solve(&p2);
    println!("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
    println!("{p3}");
    */
}

fn get_or_solve(s: String, iter_nr: u32, memory: &mut HashMap<String, HashMap<u32, usize>>) -> usize {
    if let Some(v1) = memory.get(&s) {
        if let Some(v2) = v1.get(&iter_nr) {
            return *v2;
        }
    }
    
    let mut total = 0;
    if iter_nr > 0 {
        let s_next = solve(&s);
        let parts = s_next.split("AA").collect::<Vec<&str>>();
        // The last one does not have "AA"
        for i in 0..parts.len() - 1 {
            let v = get_or_solve(parts[i].to_string() + "AA", iter_nr - 1, memory);
            total += v;
        }
        let v = get_or_solve(parts[parts.len() - 1].to_string(), iter_nr - 1, memory);
        total += v;
    }
    else {
        total = s.len();
    }

    memory.entry(s).or_insert(HashMap::new()).insert(iter_nr, total);

    total
}

pub fn main_day21_task2() {
    _ = env_logger::builder().format_timestamp(None).try_init();

    // 1. pattern, 2. iterations
    let mut memory: HashMap<String, HashMap<u32, usize>> = HashMap::new();
    
    let v = vec![
        ("<^^^AvvvA^^Avv>A", 805), // 805A -> 72
        ("^<<A^^A>vvvA>A", 170), // 170A -> 64
        ("^<<A>A^^>AvvvA", 129), // 129A -> 70
        ("<^A^^Avv>AvA", 283), // 283A -> 68
        ("<^^A<A>vvA>A", 540), // 540A -> 68
    ];

    let iter_nr = 25;
    let mut result = 0;
    for (s, nr) in v.iter() {
        get_or_solve(s.to_string(), iter_nr, &mut memory);
        result += *nr * memory.get(*s).unwrap().get(&iter_nr).unwrap();
        log::debug!("{} {}", nr, memory.get(*s).unwrap().get(&iter_nr).unwrap());
    }
    println!("result={}", result);
}
