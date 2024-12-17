use std::{cmp::Ordering, collections::HashMap, fs};

use itertools::Itertools;

fn parse_input(lines: &Vec<&str>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules = Vec::<String>::new();
    let mut page_updates = Vec::<Vec<i32>>::new();
    let empty_line_pos = lines.iter().find_position(|line| **line == "").unwrap().0;

    for i in 0..lines.len() {
        if i < empty_line_pos {
            rules.push(lines[i].to_string());
        } else if i > empty_line_pos {
            page_updates.push(
                lines[i]
                    .split(",")
                    .map(|page| page.parse::<i32>().unwrap_or(0))
                    .collect_vec(),
            );
        }
    }

    return (parse_rules(rules), page_updates);
}

fn parse_rules(rules: Vec<String>) -> HashMap<i32, Vec<i32>> {
    let mut rule_map = HashMap::<i32, Vec<i32>>::new();

    for rule in rules {
        let (src, dest) = rule.split_once("|").unwrap();
        let src = src.parse::<i32>().unwrap_or(0);
        let dest = dest.parse::<i32>().unwrap_or(0);

        if let Some(rules_list) = rule_map.get_mut(&src) {
            rules_list.push(dest);
        } else {
            rule_map.insert(src, vec![dest]);
        }
    }

    return rule_map;
}

fn verify_correct_order(
    destinations: &Vec<i32>,
    page_update_left: &[i32],
    page_update_right: &[i32],
) -> bool {
    for page in page_update_right {
        if destinations.iter().find(|dest| *dest == page).is_none() {
            return false;
        }
    }

    for page in page_update_left {
        if destinations.iter().find(|dest| *dest == page).is_some() {
            return false;
        }
    }

    return true;
}

fn get_middle_number(page_update: &[i32]) -> i32 {
    let middle = (page_update.len() as f64 / 2f64).floor();

    return page_update[middle as usize];
}

fn verify_update_is_correct(page_update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut is_page_update_correct = true;

    for i in 0..page_update.len() {
        let page = page_update[i];

        if let Some(page_rules) = rules.get(&page) {
            is_page_update_correct = is_page_update_correct
                && verify_correct_order(page_rules, &page_update[..i], &page_update[i + 1..]);
        }
    }

    return is_page_update_correct;
}

fn solve_part_one(lines: &Vec<&str>) -> i32 {
    let mut result = 0;
    let (rules, page_updates) = parse_input(lines);

    for page_update in &page_updates {
        if verify_update_is_correct(page_update, &rules) {
            result += get_middle_number(page_update);
        }
    }

    return result;
}

fn compare_by_rules(a: i32, b: i32, rules: &HashMap<i32, Vec<i32>>) -> Ordering {
    if let Some(rules_a) = rules.get(&a) {
        if rules_a.contains(&b) {
            return Ordering::Less;
        } else if let Some(rules_b) = rules.get(&b) {
            if rules_b.contains(&a) {
                return Ordering::Greater;
            }
        }
    }

    return Ordering::Equal;
}

fn solve_part_two(lines: &Vec<&str>) -> i32 {
    let (rules, page_updates) = parse_input(lines);

    return page_updates
        .iter()
        .filter(|page_update| !verify_update_is_correct(page_update, &rules))
        .map(|page_update| {
            let mut owned_page_update = page_update.to_owned();
            owned_page_update.sort_by(|a, b| compare_by_rules(*a, *b, &rules));

            return get_middle_number(&owned_page_update);
        })
        .sum();
}

pub fn solve_day_five() -> (i32, i32) {
    let file = fs::read_to_string("./src/input_files/day_five.txt").unwrap();
    let lines = file.lines().collect_vec();

    return (solve_part_one(&lines), solve_part_two(&lines));
}
