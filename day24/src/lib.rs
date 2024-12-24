use std::collections::HashMap;

use env_logger;
use log;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

use utils;

lazy_static!{
    pub static ref VAR: Regex = Regex::new(r"(?<var>[a-z0-9]{3}): (?<value>[0,1]{1})").unwrap();
    pub static ref EXPR: Regex = Regex::new(r"(?<var1>[a-z0-9]{3}) (?<op>(XOR|OR|AND)) (?<var2>[a-z0-9]{3}) -> (?<var3>[a-z0-9]{3})").unwrap();
}

#[derive(Debug, PartialEq)]
enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    fn from_string(s: &str) -> Self {
        match s {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Expr {
    var1: String,
    var2: String,
    var3: String,
    op: Op,
}

impl Expr {
    fn solve(&self, vars: &mut HashMap<String, u8>, exprs: &HashMap<String, Expr>) -> u8 {
        log::debug!("\t{} {}", self.var1, vars.contains_key(&self.var1));
        let v1: u8 = match vars.get(&self.var1) {
            Some(v) => *v,
            None => {
                log::debug!("\t{:?}", exprs.get(&self.var1).unwrap());
                exprs.get(&self.var1).unwrap().solve(vars, &exprs)
            },
        };
        log::debug!("\t{} {}", self.var2, vars.contains_key(&self.var2));
        let v2: u8 = match vars.get(&self.var2) {
            Some(v) => *v,
            None => exprs.get(&self.var2).unwrap().solve(vars, &exprs),
        };
        log::debug!("\t{} {}", self.var3, vars.contains_key(&self.var3));
        let v3 = match self.op {
            Op::AND => v1 & v2,
            Op::OR => v1 | v2,
            Op::XOR => v1 ^ v2,
        };
        vars.insert(self.var3.clone(), v3);
        v3
    }
}

fn get_data(filename: &str) -> (HashMap<String, u8>, HashMap<String, Expr>) {
    let mut vars = HashMap::new();
    let mut exprs = HashMap::new();

    for line in utils::file_to_iter(filename) {
        if let Some(caps) = VAR.captures(&line) {
            vars.insert(caps["var"].to_string(), caps["value"].parse::<u8>().unwrap());
        }
        else if let Some(caps) = EXPR.captures(&line) {
            exprs.insert(
                caps["var3"].to_string(),
                Expr {
                    var1: caps["var1"].to_string(),
                    var2: caps["var2"].to_string(),
                    var3: caps["var3"].to_string(),
                    op: Op::from_string(&caps["op"]),
                }
            );
        }
    }

    (vars, exprs)
}

fn solve1(filename: &str) -> isize {
    let (mut vars, exprs) = get_data(filename);
    log::debug!("{:?}", vars);
    log::debug!("{:?}", exprs);

    for (var3, expr) in exprs.iter() {
        log::debug!("{} {:?}", var3, expr);
        expr.solve(&mut vars, &exprs);
    }
    log::debug!("{:?}", vars);

    let zs: HashMap<String, u8> = vars
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    log::debug!("{:?}", zs);

    let s: Vec<u8> = zs.iter().sorted().rev().map(|(_, v)| *v).collect();
    log::debug!("s={:?}", s);
    
    isize::from_str_radix(&itertools::join(s, ""), 2).unwrap()
}

pub fn main_day24_task1() {
    _ = env_logger::builder().format_timestamp(None).try_init();

    let sum = solve1("day24\\data.txt");

    println!("Day 24 task 1 result is {}", sum);
}

pub fn main_day24_task2() {
    // Based on https://www.reddit.com/r/adventofcode/comments/1hla5ql/2024_day_24_part_2_a_guide_on_the_idea_behind_the/

    _ = env_logger::builder().format_timestamp(None).try_init();

    let (_vars, exprs) = get_data("day24\\data.txt");

    let mut z_not_xor = vec![];
    for (var3, expr) in exprs.iter() {
        if var3.starts_with("z") && expr.op != Op::XOR {
            z_not_xor.push(var3.clone());
        }
    }
    log::debug!("{:?}", z_not_xor);

    let mut middle = vec![];
    for (var3, expr) in exprs.iter() {
        if var3.starts_with("z") { continue }
        if expr.var1.starts_with("x") || expr.var1.starts_with("y") { continue }
        if expr.var2.starts_with("x") || expr.var2.starts_with("y") { continue }
        if expr.op == Op::XOR {
            middle.push(var3.clone());
        }
    }
    log::debug!("{:?}", middle);

    log::debug!("********3");
    for (var3, expr) in exprs.iter() {
        if expr.op == Op::AND {
            for (var3b, exprb) in exprs.iter() {
                if *var3b == *var3 { continue }
                if *exprb.var1 != *var3 && *exprb.var2 != *var3 { continue }
                if exprb.op == Op::OR { continue }
                log::debug!("{var3} {var3b}");
            }
        }
    }

    log::debug!("********4");
    /*
    If you have a XOR gate with inputs x, y, there must be another XOR
    gate with this gate as an input. Search through all gates for an
    XOR-gate with this gate as an input; if it does not exist, your
    (original) XOR gate is faulty. 
    */
    for (var3, expr) in exprs.iter() {
        if expr.op != Op::XOR { continue }
        if !expr.var1.starts_with("x") && !expr.var1.starts_with("y") { continue }
        if !expr.var2.starts_with("x") && !expr.var2.starts_with("y") { continue }
        let mut has_xor = false;
        for (var3b, exprb) in exprs.iter() {
            if *var3b == *var3 { continue }
            if *exprb.var1 == *var3 || *exprb.var2 == *var3 {
                if exprb.op == Op::XOR { has_xor = true; }
            }
        }
        if !has_xor {
            log::debug!("\t{}", var3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve1("data_sample.txt"), 2024);
    }
}
