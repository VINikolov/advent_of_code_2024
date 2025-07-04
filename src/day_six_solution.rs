use std::fs;

use itertools::Itertools;

fn parse_input(file: &str) -> Vec<Vec<char>> {
    return file
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
}

fn get_guard_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                return (i, j);
            }
        }
    }

    return (0, 0);
}

fn walk(x: usize, y: usize, (dir_x, dir_y): (isize, isize), map: &mut Vec<Vec<char>>) -> i32 {
    let current_item = map[x][y];
    let (new_x, new_y) = ((x as isize) + dir_x, (y as isize) + dir_y);

    if new_x < 0 || new_x >= map.len() as isize {
        return if current_item == 'X' { 0 } else { 1 };
    }

    if new_y < 0 || new_y >= map[0].len() as isize {
        return if current_item == 'X' { 0 } else { 1 };
    }

    let new_x = new_x as usize;
    let new_y = new_y as usize;

    if map[new_x][new_y] == '#' {
        let (new_dir_x, new_dir_y) = match (dir_x, dir_y) {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => (0, 0),
        };

        return walk(x, y, (new_dir_x, new_dir_y), map);
    }

    map[x][y] = 'X';

    return if map[new_x][new_y] == 'X' { 0 } else { 1 } + walk(new_x, new_y, (dir_x, dir_y), map);
}

fn solve_part_one(map: &Vec<Vec<char>>) -> i32 {
    let (guard_pos_x, guard_pos_y) = get_guard_pos(map);
    let mut owned_mutable_map = map.clone();

    return walk(guard_pos_x, guard_pos_y, (-1, 0), &mut owned_mutable_map);
}

pub fn solve_day_six() -> (i32, i32) {
    let file = fs::read_to_string("./src/input_files/day_six.txt").unwrap();
    let map = parse_input(&file);

    return (solve_part_one(&map), 0);
}
