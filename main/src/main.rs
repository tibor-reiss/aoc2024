use day1;
use day2;
use day3;
use day4;
use day5;

fn main() {
    println!("AOC2024");

    //day1::main_day1_task1();
    //day1::main_day1_task2();

    //day2::main_day2_task1();
    //day2::main_day2_task2();

    //day3::main_day3_task1(); // 161_085_926
    //day3::main_day3_task2(); // 82_045_421

    //day4::main_day4_task1();
    //day4::main_day4_task2();

    let wrong_pages = day5::main_day5_task1(); // 4959
    day5::main_day5_task2(&wrong_pages);
}
