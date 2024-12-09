use std::fs;

use itertools::Itertools;

fn check_for_match(
    matrix: &Vec<&str>,
    x: isize,
    y: isize,
    direction_modifier: &[isize; 2],
    char_to_match: char,
) -> bool {
    if x < 0
        || x.try_into().unwrap_or(0) == matrix.len()
        || y < 0
        || y.try_into().unwrap_or(0) == matrix[0].len()
    {
        return false;
    }

    let new_char = match char_to_match {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => 'N',
    };

    if char_to_match == 'N' {
        return false;
    }

    if matrix
        .get(x as usize)
        .unwrap_or(&"")
        .chars()
        .nth(y as usize)
        .unwrap_or('N')
        == char_to_match
    {
        if char_to_match == 'S' {
            return true;
        }

        let (new_x, new_y) = (x + direction_modifier[0], y + direction_modifier[1]);

        return check_for_match(matrix, new_x, new_y, direction_modifier, new_char);
    }

    return false;
}

fn solve_part_one(input: &Vec<&str>) -> i32 {
    let mut result = 0;
    let direction_modifiers = vec![
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1],
        [-1, 1],
        [1, 1],
        [-1, -1],
        [1, -1],
    ];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i].chars().nth(j).unwrap() == 'X' {
                for modifier in &direction_modifiers {
                    if check_for_match(input, i as isize, j as isize, modifier, 'X') {
                        result += 1;
                    }
                }
            }
        }
    }

    return result;
}

fn check_diagonal(
    matrix: &Vec<&str>,
    top_coords: (isize, isize),
    bottom_coords: (isize, isize),
) -> bool {
    let (top_x, top_y) = top_coords;
    let (bottom_x, bottom_y) = bottom_coords;
    let upper_left = matrix
        .get(top_x as usize)
        .unwrap_or(&"")
        .chars()
        .nth(top_y as usize)
        .unwrap_or('N');
    let lower_right = matrix
        .get(bottom_x as usize)
        .unwrap_or(&"")
        .chars()
        .nth(bottom_y as usize)
        .unwrap_or('N');

    return match (upper_left, lower_right) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    };
}

fn check_for_mas_match(matrix: &Vec<&str>, i: isize, j: isize) -> bool {
    return check_diagonal(matrix, (i - 1, j - 1), (i + 1, j + 1))
        && check_diagonal(matrix, (i - 1, j + 1), (i + 1, j - 1));
}

fn solve_part_two(input: &Vec<&str>) -> i32 {
    let mut result = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i].chars().nth(j).unwrap() == 'A' {
                if check_for_mas_match(input, i as isize, j as isize) {
                    result += 1;
                }
            }
        }
    }

    return result;
}

pub fn solve_day_four() -> (i32, i32) {
    let file = fs::read_to_string("./src/input_files/day_four.txt").unwrap();
    let input = file.lines().collect_vec();

    return (solve_part_one(&input), solve_part_two(&input));
}
