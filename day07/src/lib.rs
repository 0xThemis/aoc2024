use std::result;

use aoc_traits::AdventOfCodeDay;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = u64;
    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.lines().map(solve_equation).sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.lines().map(testitest).sum()
    }

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
    }
}

fn testitest(equation: &str) -> u64 {
    let mut split = equation.split(':');
    let equation_result = split.next().unwrap().parse::<u64>().unwrap();

    let numbers = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let amount_numbers = numbers.len();
    let masks = 1 << amount_numbers - 1;

    for mask in 0..masks {
        let concats = concat_and_solve_equation(&numbers, mask);
        let result = solve_equation_iter(equation_result, concats.into_iter());
        if result != 0 {
            return result;
        }
    }
    0
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn concat_and_solve_equation(numbers: &[u64], mut concat_mask: usize) -> Vec<u64> {
    let mut index = 0;
    let mut current_number = 0;
    let mut this_iter = vec![];
    loop {
        if index >= numbers.len() {
            break;
        }
        let concatinated = concat(current_number, numbers[index]);
        current_number = concatinated;
        if concat_mask & 1 == 1 {
            // concat
            current_number = concatinated;
        } else {
            this_iter.push(current_number);
            current_number = 0;
        }
        concat_mask >>= 1;
        index += 1;
    }
    this_iter
}

fn solve_equation_iter(equation_result: u64, mut numbers: impl Iterator<Item = u64>) -> u64 {
    let mut current_level = Vec::with_capacity(2_usize << 14);
    let mut next_level = Vec::with_capacity(2_usize << 14);
    //first level
    let init = numbers.next().unwrap();
    current_level.push(init);
    for next_umber in numbers {
        for last_number in current_level.iter() {
            let add = last_number + next_umber;
            let mul = last_number * next_umber;
            if add == equation_result || mul == equation_result {
                return equation_result;
            }
            next_level.push(add);
            next_level.push(mul);
        }
        std::mem::swap(&mut current_level, &mut next_level);
        next_level.clear();
    }
    0
}

fn solve_equation(equation: &str) -> u64 {
    let mut split = equation.split(':');
    let equation_result = split.next().unwrap().parse().unwrap();
    let mut current_level = Vec::with_capacity(2_usize << 14);
    let mut next_level = Vec::with_capacity(2_usize << 14);
    let mut numbers = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    //first level
    let init = numbers.next().unwrap();
    current_level.push(init);
    for next_umber in numbers {
        for last_number in current_level.iter() {
            let add = last_number + next_umber;
            let mul = last_number * next_umber;
            if add == equation_result || mul == equation_result {
                return equation_result;
            }
            next_level.push(add);
            next_level.push(mul);
        }
        std::mem::swap(&mut current_level, &mut next_level);
        next_level.clear();
    }
    0
}

#[test]
fn day07() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/demo1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    //assert_eq!(42283209483350, Solver::solve_part1(&parsed));
    assert_eq!(1482, Solver::solve_part2(&parsed));
}
