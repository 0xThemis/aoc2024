use std::cmp::Ordering;

use aoc_traits::AdventOfCodeDay;

type Input = Vec<Vec<i64>>;
#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Input;
    type Part1Output = usize;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        solve_puzzle1(input)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        solve_puzzle2(input)
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        parse_demo1(input)
    }
}
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

fn solve_puzzle1(reports: &Input) -> usize {
    let mut safe_reports = 0;
    for report in reports.iter() {
        if is_safe(report) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn solve_puzzle2(reports: &Input) -> usize {
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

#[test]
fn day02() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(559, Solver::solve_part1(&parsed));
    assert_eq!(601, Solver::solve_part2(&parsed));
}
