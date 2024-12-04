use std::fs;

use itertools::Itertools;
use regex::Regex;

fn multiply_numbers(input: &str) -> i32 {
    let mut result = 0;
    let tokens = input.split("mul").collect_vec();
    let correct_token_regex = Regex::new(r"^\(\d{1,3},\d{1,3}\)").unwrap();

    for token in tokens {
        let matched_token = correct_token_regex.find(token);

        if let Some(value) = matched_token {
            let value_str = value.as_str();
            let numbers = &value_str[1..value_str.len() - 1].split_once(",").unwrap();
            result += numbers.0.parse::<i32>().unwrap() * numbers.1.parse::<i32>().unwrap();
        }
    }

    return result;
}

fn solve_part_two(input: &str) -> i32 {
    let mut result = 0;
    let do_sections = input.split("do()").collect_vec();

    for section in do_sections {
        let dont_sections = section.split_once("don't()");

        if let Some(values) = dont_sections {
            result += multiply_numbers(values.0);
        } else {
            result += multiply_numbers(section);
        }
    }

    return result;
}

pub fn solve_day_three() -> (i32, i32) {
    let input = fs::read_to_string("./src/input_files/day_three.txt").unwrap();

    return (multiply_numbers(&input), solve_part_two(&input));
}
