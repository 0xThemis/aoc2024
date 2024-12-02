use std::{cmp::Ordering, result};

fn parse_demo1(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i64]) -> bool {
    let mut order = Ordering::Equal;
    for levels in report.windows(2) {
        let lhs = levels[0];
        let rhs = levels[1];
        let diff = (lhs - rhs).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        match (order, lhs.cmp(&rhs)) {
            (Ordering::Equal, Ordering::Less) => order = Ordering::Less,
            (Ordering::Equal, Ordering::Greater) => order = Ordering::Greater,
            (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less) => {
                return false;
            }
            _ => continue,
        }
    }
    true
}

fn solve_puzzle1(input: &str) -> usize {
    let reports = parse_demo1(input);
    let mut safe_reports = 0;
    for report in reports.iter() {
        if is_safe(report) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn solve_puzzle2(input: &str) -> usize {
    let reports = parse_demo1(input);
    let mut safe_reports = 0;
    for report in reports.iter() {
        if is_safe(report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut removed_level = report.clone();
                removed_level.remove(i);
                if is_safe(&removed_level) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

fn main() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let result = solve_puzzle2(input.trim());
    println!("{result}")
}

#[test]
fn demo1() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/demo1.txt")).unwrap();
    assert_eq!(2, solve_puzzle1(input.trim()));
}

#[test]
fn demo2() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/demo1.txt")).unwrap();
    assert_eq!(4, solve_puzzle2(input.trim()));
}

#[test]
fn puzzle1() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    assert_eq!(559, solve_puzzle1(input.trim()));
}
