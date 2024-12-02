use std::fs;

use itertools::Itertools;

fn parse_input(file: &str) -> (Vec<i32>, Vec<i32>) {
    let values = file
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(first, second)| {
            (
                first.trim().parse::<i32>().unwrap(),
                second.trim().parse::<i32>().unwrap(),
            )
        })
        .collect_vec();

    let mut left_values = Vec::<i32>::new();
    let mut right_values = Vec::<i32>::new();

    values.iter().for_each(|(left, right)| {
        left_values.push(*left);
        right_values.push(*right);
    });

    return (left_values, right_values);
}

fn get_sum_of_distances(mut left_values: Vec<i32>, mut right_values: Vec<i32>) -> i32 {
    left_values.sort_unstable();
    right_values.sort_unstable();

    return left_values
        .iter()
        .enumerate()
        .map(|(idx, value)| (value - right_values[idx]).abs())
        .sum();
}

fn get_number_of_occurances(values: &Vec<i32>, value: i32) -> usize {
    return values.iter().filter(|v| **v == value).count();
}

fn get_similarity_score(left_values: Vec<i32>, right_values: Vec<i32>) -> i32 {
    return left_values
        .iter()
        .map(|value| {
            return value * i32::try_from(get_number_of_occurances(&right_values, *value)).unwrap();
        })
        .sum();
}

pub fn solve_day_one() -> (i32, i32) {
    let file = fs::read_to_string("./src/input_files/day_one.txt").unwrap();

    let (left_values, right_values) = parse_input(&file);

    return (
        get_sum_of_distances(left_values.clone(), right_values.clone()),
        get_similarity_score(left_values, right_values),
    );
}
