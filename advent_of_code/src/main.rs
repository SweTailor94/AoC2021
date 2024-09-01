mod input_reader;
mod day_1;
mod day_2;
mod day_3;

use crate::day_1::day_1_solution::run_day_1;
use crate::day_2::day_2_solution::run_day_2;
use crate::day_3::day_3_solution::run_day_3;

fn main() {
    println!("Hello, Advent of Code 2021!");

    run_day_1();
    run_day_2();
    run_day_3();
}
