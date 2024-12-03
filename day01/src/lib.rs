use aoc_traits::AdventOfCodeDay;

type Part1Input = (Vec<usize>, Vec<usize>);
type Part2Input = (Vec<usize>, Vec<usize>);
#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = (Part1Input, Part2Input);
    type Part1Output = i64;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let (input, _) = input;
        solve_puzzle1(input)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let (_, input) = input;
        solve_puzzle2(input)
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        let input_1 = parse_input1(input);
        let input_2 = parse_input2(input);
        (input_1, input_2)
    }
}

fn parse_input1(str: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut lhs, mut rhs) = str
        .lines()
        .map(|line| {
            let mut chars = line.split_whitespace();
            let left = chars.next().unwrap().parse::<usize>().unwrap();
            let right = chars.next_back().unwrap().parse::<usize>().unwrap();
            (left, right)
        })
        .collect::<(Vec<_>, Vec<_>)>();
    lhs.sort();
    rhs.sort();
    (lhs, rhs)
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

fn solve_puzzle1(input: &Part1Input) -> i64 {
    let (lhs, rhs) = input;
    lhs.into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| (*lhs as i64 - *rhs as i64).abs())
        .sum()
}

fn solve_puzzle2(input: &Part2Input) -> usize {
    let (lhs, rhs) = input;
    lhs.into_iter().map(|x| x * rhs[*x]).sum()
}

#[test]
fn day01() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(2113135, Solver::solve_part1(&parsed));
    assert_eq!(19097157, Solver::solve_part2(&parsed));
}
