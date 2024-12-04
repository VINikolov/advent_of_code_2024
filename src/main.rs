// mod day_one_solution;
// mod day_two_solution;
mod day_three_solution;

// use crate::day_one_solution::solve_day_one;
// use crate::day_two_solution::solve_day_two;
use crate::day_three_solution::solve_day_three;

fn main() {
    let (answer_one, answer_two) = solve_day_three();

    println!("Solution for problem one: {answer_one}");
    println!("Solution for problem two: {answer_two}");
}
