use aoc_traits::AdventOfCodeDay;

pub struct Day1Solver;

impl<'a> AdventOfCodeDay<'a> for Day1Solver {
    type ParsedInput = (Vec<usize>, Vec<usize>);

    type Part1Output = i64;

    type Part2Output = usize;

    fn solve_part1(_input: &Self::ParsedInput) -> Self::Part1Output {
        todo!()
    }

    fn solve_part2(_input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
            .lines()
            .map(|line| {
                let mut chars = line.split_whitespace();
                let left = chars.next().unwrap().parse::<usize>().unwrap();
                let right = chars.next_back().unwrap().parse::<usize>().unwrap();
                (left, right)
            })
            .collect()
    }
}

fn parse_input1(str: &str) -> (Vec<usize>, Vec<usize>) {
    str.lines()
        .map(|line| {
            let mut chars = line.split_whitespace();
            let left = chars.next().unwrap().parse::<usize>().unwrap();
            let right = chars.next_back().unwrap().parse::<usize>().unwrap();
            (left, right)
        })
        .collect()
}

fn parse_input2(str: &str) -> (Vec<usize>, Vec<usize>) {
    let mut vec = Vec::with_capacity(1000);
    let mut count = vec![0; 100000];
    for line in str.lines() {
        let mut chars = line.split_whitespace();
        let left = chars.next().unwrap().parse::<usize>().unwrap();
        let right = chars.next_back().unwrap().parse::<usize>().unwrap();
        count[right] += 1;
        vec.push(left);
    }
    (vec, count)
}

fn solve_puzzle1(input: &str) -> i64 {
    let (mut lhs, mut rhs) = parse_input1(input);
    lhs.sort();
    rhs.sort();
    lhs.into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| (lhs as i64 - rhs as i64).abs())
        .sum()
}

fn solve_puzzle2(input: &str) -> usize {
    let (lhs, rhs) = parse_input2(input);
    lhs.into_iter().map(|x| x * rhs[x]).sum()
}

fn main() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let puzzle1 = solve_puzzle1(input.trim());
    let puzzle2 = solve_puzzle2(input.trim());
    println!("puzzle1: {puzzle1}");
    println!("puzzle2: {puzzle2}");
}

#[test]
fn demo1() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/demo1.txt")).unwrap();
    assert_eq!(11, solve_puzzle1(input.trim()));
}

#[test]
fn demo2() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/demo1.txt")).unwrap();
    assert_eq!(31, solve_puzzle2(input.trim()));
}
