use day1;
use day2;
use day3;
use day4;
use day5;
use day6;
use day7;
use day8;

fn main() {
    println!("AOC2024");

    day1::main_day1_task1(); // 1938424
    day1::main_day1_task2(); // 22014209

    day2::main_day2_task1(); // 559
    day2::main_day2_task2(); // 601

    day3::main_day3_task1(); // 161_085_926
    day3::main_day3_task2(); // 82_045_421

    day4::main_day4_task1(); // 2521
    day4::main_day4_task2(); // 1912

    let wrong_pages = day5::main_day5_task1(); // 4959
    day5::main_day5_task2(&wrong_pages); // 4655

    day6::main_day6_task1(); // 4819
    day6::main_day6_task2(); // 1796

    day7::main_day7_task1(); // 3598800864292
    day7::main_day7_task2(); // 340362529351427

    day8::main_day8_task1(); // 413
    day8::main_day8_task2(); // 1417
}
