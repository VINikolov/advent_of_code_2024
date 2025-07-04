use std::fs;

use itertools::Itertools;

fn parse_line(line: (&str, &str)) -> (i64, Vec<i64>) {
    return (
        line.0.parse::<i64>().unwrap_or(0),
        line.1
            .split_whitespace()
            .map(|value| value.parse::<i64>().unwrap_or(0))
            .collect_vec(),
    );
}

fn parse_input(file: &str) -> Vec<(i64, Vec<i64>)> {
    return file
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(parse_line)
        .collect_vec();
}

fn solve_equation(idx: usize, current_value: i64, values: &Vec<i64>, target: &i64) -> bool {
    if idx + 1 >= values.len() {
        return current_value == *target;
    }

    if current_value > *target {
        return false;
    }

    let idx = idx + 1;

    return solve_equation(idx, current_value + values[idx], values, target)
        || solve_equation(idx, current_value * values[idx], values, target);
}

fn check_equation((result, values): &(i64, Vec<i64>)) -> bool {
    return solve_equation(0, values[0], values, result);
}

fn solve_part_one(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    return input
        .iter()
        .filter(|line| check_equation(*line))
        .map(|line| line.0)
        .sum();
}

fn concat_numbers(a: i64, b: i64) -> i64 {
    return a as i64 * 10i64.pow(b.ilog10() + 1) + b as i64;
}

fn solve_equation_with_number_merge(
    idx: usize,
    current_value: i64,
    values: &Vec<i64>,
    target: &i64,
) -> bool {
    if idx + 1 >= values.len() {
        return current_value == *target;
    }

    if current_value > *target {
        return false;
    }

    let idx = idx + 1;

    return solve_equation_with_number_merge(idx, current_value + values[idx], values, target)
        || solve_equation_with_number_merge(idx, current_value * values[idx], values, target)
        || solve_equation_with_number_merge(
            idx,
            concat_numbers(current_value, values[idx]),
            values,
            target,
        );
}

fn check_equation_with_number_merge((result, values): &(i64, Vec<i64>)) -> bool {
    return solve_equation_with_number_merge(0, values[0], values, result);
}

fn solve_part_two(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    return input
        .iter()
        .filter(|line| check_equation_with_number_merge(*line))
        .map(|line| line.0)
        .sum();
}

pub fn solve_day_seven() -> (i64, i64) {
    let file = fs::read_to_string("./src/input_files/day_seven.txt").unwrap();
    let input = parse_input(&file);

    return (solve_part_one(&input), solve_part_two(&input));
}
