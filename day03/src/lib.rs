use std::{collections::VecDeque, i64};

use aoc_traits::AdventOfCodeDay;
use regex::Regex;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = i64;
    type Part2Output = i64;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        solve_puzzle1(input)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        solve_puzzle2(input)
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        input
    }
}

struct Mul {
    index: usize,
    lhs: i64,
    rhs: i64,
}

fn solve_puzzle1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|needle| {
            let (_, [a, b]) = needle.extract();
            a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()
        })
        .sum()
}

fn solve_puzzle2(hay: &str) -> i64 {
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut opcodes = VecDeque::with_capacity(1024);
    mul.captures_iter(hay).for_each(|needle| {
        let (_, [lhs, rhs]) = needle.extract();
        opcodes.push_back(Mul {
            index: needle.get(0).unwrap().start(),
            lhs: lhs.parse().unwrap(),
            rhs: rhs.parse().unwrap(),
        });
    });
    let mut index = 0;
    let mut result = 0;
    let mut is_enabled = true;
    loop {
        if index == hay.len() {
            break;
        }
        if hay[index..].starts_with("don't()") {
            is_enabled = false;
        } else if hay[index..].starts_with("do()") {
            is_enabled = true;
        } else {
            if let Some(opcode) = opcodes.front() {
                if opcode.index == index {
                    let opcode = opcodes.pop_front().unwrap();
                    if is_enabled {
                        result += opcode.lhs * opcode.rhs;
                    }
                }
            }
        }
        index += 1;
    }
    result
}

#[test]
fn day03() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(178794710, Solver::solve_part1(&parsed));
    assert_eq!(76729637, Solver::solve_part2(&parsed));
}
