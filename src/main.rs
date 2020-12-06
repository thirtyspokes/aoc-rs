#[macro_use]
extern crate lazy_static;

mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

fn main() {
    println!("Day one: Report Repair");
    day_one::solve_day_one();

    println!("\nDay two: Password Philosophy");
    day_two::solve_day_two();

    println!("\nDay three: Tobaggan Trajectory");
    day_three::solve_day_three();

    println!("\nDay four: Passport Processing");
    day_four::solve_day_four();

    println!("\nDay five: Binary Boarding");
    day_five::solve_day_five();

    println!("\nDay six: Custom Customs");
    day_six::solve_day_six();
}
