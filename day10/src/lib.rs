use aoc_traits::AdventOfCodeDay;

pub struct TopographicMap {
    width: usize,
    height: usize,
    map: Vec<Vec<i8>>,
    steps: Vec<Steps>,
}

#[derive(Default, Clone)]
struct Steps {
    value: i8,
    uphill: Vec<usize>,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = TopographicMap;
    type Part1Output = usize;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.solve_part1()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.solve_part2()
    }

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        parse_input(input)
    }
}

macro_rules! to_index {
    ($row:expr,$col:expr, $width:expr) => {
        ($row * $width) + $col
    };
}

impl TopographicMap {
    fn solve_part1(&self) -> usize {
        let mut counter = 0;
        for row in 0..self.width {
            for col in 0..self.height {
                if self.map[row][col] == 0 {
                    let mut vec = Vec::with_capacity(10);
                    get_mountain_tip(to_index!(row, col, self.width), &self.steps, &mut vec);
                    counter += vec.len();
                }
            }
        }
        counter
    }

    fn solve_part2(&self) -> usize {
        let mut counter = 0;
        for row in 0..self.width {
            for col in 0..self.height {
                if self.map[row][col] == 0 {
                    let mut vec = Vec::with_capacity(10);
                    get_mountain_tip2(to_index!(row, col, self.width), &self.steps, &mut vec);
                    counter += vec.len();
                }
            }
        }
        counter
    }
}

fn get_mountain_tip(idx: usize, all_steps: &[Steps], paths: &mut Vec<usize>) {
    let start = &all_steps[idx];
    if start.uphill.is_empty() && !paths.contains(&idx) && start.value == 9 {
        paths.push(idx);
    } else {
        for uphill in &start.uphill {
            get_mountain_tip(*uphill, all_steps, paths)
        }
    }
}

fn get_mountain_tip2(idx: usize, all_steps: &[Steps], paths: &mut Vec<usize>) {
    let start = &all_steps[idx];
    if start.uphill.is_empty() && start.value == 9 {
        paths.push(idx);
    } else {
        for uphill in &start.uphill {
            get_mountain_tip2(*uphill, all_steps, paths)
        }
    }
}
fn parse_input(input: &str) -> TopographicMap {
    let mut width = 0;
    let mut map = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut row = Vec::with_capacity(100);
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as i8);
        }
        row.push(i8::MAX);
        width = row.len();
        map.push(row);
    }
    map.push(vec![i8::MAX; width]);

    let height = map.len();

    let mut steps = vec![Steps::default(); width * height];
    for row in 0..width - 1 {
        for col in 0..height - 1 {
            let right = map[row][col + 1];
            let down = map[row + 1][col];
            let current = map[row][col];
            let current_idx = to_index!(row, col, width);
            let row_idx = to_index!(row + 1, col, width);
            steps[current_idx].value = current;
            // check if up
            if current - right == -1 {
                // we are going up
                steps[current_idx].uphill.push(current_idx + 1);
            } else if current - right == 1 {
                // we are going down
                steps[current_idx + 1].uphill.push(current_idx);
            }

            if current - down == -1 {
                // we are going up
                steps[current_idx].uphill.push(row_idx);
            } else if current - down == 1 {
                // we are going down
                steps[row_idx].uphill.push(current_idx);
            }
        }
    }
    TopographicMap {
        map,
        steps,
        width,
        height,
    }
}

#[test]
fn day10() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(746, Solver::solve_part1(&parsed));
    assert_eq!(1541, Solver::solve_part2(&parsed));
}
