use env_logger;
use lazy_static::lazy_static;
use log;
use regex::Regex;

use utils;

const FILENAME: &str = "day17\\data_identical.txt";
lazy_static!{
    pub static ref REGEX_A: Regex = Regex::new(r"Register A: (?<nr>\d+)").unwrap();
    pub static ref REGEX_B: Regex = Regex::new(r"Register B: (?<nr>\d+)").unwrap();
    pub static ref REGEX_C: Regex = Regex::new(r"Register C: (?<nr>\d+)").unwrap();
    pub static ref PROGRAM: Regex = Regex::new(r"Program: (?<program>.+)").unwrap();
}

struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pointer: usize,
}

impl Computer {
    fn get_real_operand(&self, opcode: u64, operand: u64) -> u64 {
        if opcode == 1 || opcode == 3 { return operand; }
        match operand {
            0 => 0,
            1 => 1,
            2 => 1,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => 7,
        }
    }
    
    fn calc(&mut self, opcode: u64, operand: u64) -> Option<u64> {
        log::debug!("pointer={} opcode={opcode} operand={operand} reg_a={} reg_b={} reg_c={}", self.pointer, self.reg_a, self.reg_b, self.reg_c);
        let mut output: Option<u64> = None;
        let operand: u64 = self.get_real_operand(opcode, operand);
        
        // Move the pointer by 2
        self.pointer += 2;

        match opcode {
            0 => self.reg_a /= 2_u64.pow(operand as u32),
            1 => self.reg_b ^= operand,
            2 => self.reg_b = operand % 8,
            3 => {
                match self.reg_a {
                    0 => (),
                    _ => self.pointer = operand as usize,
                }
            },
            4 => self.reg_b ^= self.reg_c,
            5 => output = Some(operand % 8),
            6 => self.reg_b = self.reg_a / 2_u64.pow(operand as u32),
            7 => self.reg_c = self.reg_a / 2_u64.pow(operand as u32),
            _ => (),
        };

        output
    }
}

fn get_data() -> (u64, u64, u64, Vec<u64>) {
    let mut reg_a: u64 = 0;
    let mut reg_b: u64 = 0;
    let mut reg_c: u64 = 0;
    let mut prog: Vec<u64> = vec![];
    
    for line in utils::file_to_iter(FILENAME) {
        if let Some(caps) = REGEX_A.captures(&line) {
            reg_a = caps["nr"].parse::<u64>().unwrap();
        }
        if let Some(caps) = REGEX_B.captures(&line) {
            reg_b = caps["nr"].parse::<u64>().unwrap();
        }
        if let Some(caps) = REGEX_C.captures(&line) {
            reg_c = caps["nr"].parse::<u64>().unwrap();
        }
        if let Some(caps) = PROGRAM.captures(&line) {
            prog = caps["program"].split(',').map(|c| c.parse::<u64>().unwrap()).collect();
        }
    }

    (reg_a, reg_b, reg_c, prog)
}

pub fn main_day17_task1() {
    _ = env_logger::try_init();

    let (reg_a, reg_b, reg_c, prog) = get_data();
    log::debug!("{reg_a} {reg_b} {reg_c} {:?}", prog);
    let mut computer = Computer { reg_a, reg_b, reg_c, pointer: 0 };
    let mut result: Vec<u64> = vec![];
    let mut opcode: u64;
    let mut operand: u64;

    loop {
        if computer.pointer >= prog.len() { break }
        opcode = prog[computer.pointer];
        operand = prog[computer.pointer + 1];
        if let Some(v) = computer.calc(opcode, operand) { result.push(v); }
    }

    println!("Day 17 task 1 result is {}", result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
}

pub fn main_day17_task2() {
    // Reverse engineering with pen and paper, that
    //   C has to be small, either 0 or 1 - it turned out that 1 is already too big due to the 7,5 sequence, which is C = A / 2.pow(B)
    //   B has to be small as well, due to the output of 0, it should be 0
    // So after fixing B=C=0
    //   to get 0 (last element), A has to be 5
    //   to get 3,0, A has to be 46
    // From here on it is roughly a factor of 8 due the third last sequence, 0,3, which is A = A / 2.pow(3)
    // Thus, one just needs to step by roughly a factor of 8 and - at every sub-step - check if the sub-sequence matches
    // 46 * 8.pow(14) = 202310139510784
    // solution       = 202367025818154

    _ = env_logger::try_init();

    let (reg_a, reg_b, reg_c, prog) = get_data();
    log::debug!("{reg_a} {reg_b} {reg_c} {:?}", prog);
    let mut computer = Computer { reg_a, reg_b, reg_c, pointer: 0 };
    let mut opcode: u64;
    let mut operand: u64;

    for i in 202367025818154_u64..202367025818155 {
        let mut result: Vec<u64> = vec![];
        //computer.reg_a = i;
        //computer.pointer = 0;
        loop {
            if computer.pointer >= prog.len() { break }
            opcode = prog[computer.pointer];
            operand = prog[computer.pointer + 1];
            if let Some(v) = computer.calc(opcode, operand) { result.push(v); }
        }

        let result = result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",");
        if result == "2,4,1,1,7,5,4,7,1,4,0,3,5,5,3,0" {
            println!("Day 17 task 1 result is i={i} {}", result);
        }
    }
}
