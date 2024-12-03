use std::fs;

use itertools::Itertools;

fn is_safe_decreasing(report: &Vec<i32>) -> bool {
    return match report.as_slice() {
        [] => true,
        [_] => true,
        [a, b, ..] => a - b <= 3 && a - b > 0 && is_safe_decreasing(&report[1..].to_vec()),
    };
}

fn is_safe_increasing(report: &Vec<i32>) -> bool {
    return match report.as_slice() {
        [] => true,
        [_] => true,
        [a, b, ..] => a - b >= -3 && a - b < 0 && is_safe_increasing(&report[1..].to_vec()),
    };
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    return is_safe_decreasing(&report) || is_safe_increasing(&report);
}

fn is_report_safe_with_tolerance(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut fixed_report = report.clone();
        fixed_report.remove(i);

        if is_report_safe(&fixed_report) {
            return true;
        }
    }

    return false;
}

fn get_safe_reports_count<F>(reports: &Vec<Vec<i32>>, is_report_safe_fn: F) -> usize
where
    F: Fn(&Vec<i32>) -> bool,
{
    let safe_report_flags = reports
        .iter()
        .map(|report| is_report_safe_fn(report))
        .collect_vec();

    return safe_report_flags.iter().filter(|flag| **flag).count();
}

pub fn solve_day_two() -> (usize, usize) {
    let reports = fs::read_to_string("./src/input_files/day_two.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    return (
        get_safe_reports_count(&reports, is_report_safe),
        get_safe_reports_count(&reports, is_report_safe_with_tolerance),
    );
}
