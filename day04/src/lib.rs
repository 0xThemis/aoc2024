use aoc_traits::AdventOfCodeDay;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Vec<u8>>;
    type Part1Output = u32;
    type Part2Output = u32;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let column_width = input.len() - 3;
        let row_width = input[0].len() - 3;
        let mut result = 0;
        for x in 3..column_width {
            for y in 3..row_width {
                if input[x][y] == b'X' {
                    result += is_mas(input[x + 1][y], input[x + 2][y], input[x + 3][y]);
                    result += is_mas(
                        input[x + 1][y + 1],
                        input[x + 2][y + 2],
                        input[x + 3][y + 3],
                    );
                    result += is_mas(input[x][y + 1], input[x][y + 2], input[x][y + 3]);
                    result += is_mas(
                        input[x - 1][y + 1],
                        input[x - 2][y + 2],
                        input[x - 3][y + 3],
                    );
                    result += is_mas(input[x - 1][y], input[x - 2][y], input[x - 3][y]);
                    result += is_mas(
                        input[x - 1][y - 1],
                        input[x - 2][y - 2],
                        input[x - 3][y - 3],
                    );
                    result += is_mas(input[x][y - 1], input[x][y - 2], input[x][y - 3]);
                    result += is_mas(
                        input[x + 1][y - 1],
                        input[x + 2][y - 2],
                        input[x + 3][y - 3],
                    );
                }
            }
        }
        result
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let column_width = input.len() - 3;
        let row_width = input[0].len() - 3;
        let mut result = 0;
        for x in 3..column_width {
            for y in 3..row_width {
                if input[x][y] == b'A' {
                    let mut left = is_mas(input[x - 1][y - 1], input[x][y], input[x + 1][y + 1]);
                    left += is_mas(input[x + 1][y + 1], input[x][y], input[x - 1][y - 1]);
                    if left == 1 {
                        let mut right =
                            is_mas(input[x - 1][y + 1], input[x][y], input[x + 1][y - 1]);
                        right += is_mas(input[x + 1][y - 1], input[x][y], input[x - 1][y + 1]);
                        if right == 1 {
                            result += 1
                        }
                    }
                }
            }
        }
        result
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        parse_input(input)
    }
}

fn is_mas(m: u8, a: u8, s: u8) -> u32 {
    if m == b'M' && a == b'A' && s == b'S' {
        1
    } else {
        0
    }
}

fn parse_row(line: &str, width: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(width);
    result.push(0);
    result.push(0);
    result.push(0);
    for char in line.bytes() {
        result.push(char);
    }
    result.push(0);
    result.push(0);
    result.push(0);
    result
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let width = first_line.len();
    let mut field = Vec::with_capacity(1024);
    let empty_row = vec![0; width + 6];
    field.push(empty_row.clone());
    field.push(empty_row.clone());
    field.push(empty_row.clone());
    field.push(parse_row(first_line, width));
    for line in lines {
        field.push(parse_row(line, width));
    }
    field.push(empty_row.clone());
    field.push(empty_row.clone());
    field.push(empty_row);
    field
}

#[test]
fn day04() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(2613, Solver::solve_part1(&parsed));
    assert_eq!(1905, Solver::solve_part2(&parsed));
}
