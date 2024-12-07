use aoc_traits::AdventOfCodeDay;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = u64;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.lines().map(solve_equation).sum()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        0
    }

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
    }
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
    for nnext_umber in numbers {
        for last_number in current_level.iter() {
            let add = last_number + nnext_umber;
            let mul = last_number * nnext_umber;
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
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(42283209483350, Solver::solve_part1(&parsed));
    //^assert_eq!(1482, Solver::solve_part2(&parsed));
}
