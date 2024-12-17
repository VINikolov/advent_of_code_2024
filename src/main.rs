// mod day_one_solution;
// mod day_two_solution;
// mod day_three_solution;
// mod day_four_solution;
mod day_five_solution;

// use crate::day_one_solution::solve_day_one;
// use crate::day_two_solution::solve_day_two;
// use crate::day_three_solution::solve_day_three;
// use crate::day_four_solution::solve_day_four;
use crate::day_five_solution::solve_day_five;

fn main() {
    let (answer_one, answer_two) = solve_day_five();

    println!("Solution for problem one: {answer_one}");
    println!("Solution for problem two: {answer_two}");
}
