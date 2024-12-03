use aoc_traits::AdventOfCodeDay;

type Input = (Vec<usize>, Vec<usize>, Vec<usize>);
#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Input;
    type Part1Output = i64;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        solve_puzzle1(input)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        solve_puzzle2(input)
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        parse_input(input)
    }
}

fn parse_input(str: &str) -> Input {
    let mut count = vec![0; 100000];
    let (mut lhs, mut rhs) = str
        .lines()
        .map(|line| {
            let mut chars = line.split_whitespace();
            let left = chars.next().unwrap().parse::<usize>().unwrap();
            let right = chars.next_back().unwrap().parse::<usize>().unwrap();
            count[right] += 1;
            (left, right)
        })
        .collect::<(Vec<_>, Vec<_>)>();
    lhs.sort();
    rhs.sort();
    (lhs, rhs, count)
}

fn solve_puzzle1(input: &Input) -> i64 {
    let (lhs, rhs, _) = input;
    lhs.into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| (*lhs as i64 - *rhs as i64).abs())
        .sum()
}

fn solve_puzzle2(input: &Input) -> usize {
    let (lhs, _, count) = input;
    lhs.into_iter().map(|x| x * count[*x]).sum()
}

#[test]
fn day01() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(2113135, Solver::solve_part1(&parsed));
    assert_eq!(19097157, Solver::solve_part2(&parsed));
}
