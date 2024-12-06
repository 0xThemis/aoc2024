use core::panic;
use std::usize;

use aoc_traits::AdventOfCodeDay;

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;
const NO_WALL: usize = usize::MAX;

#[derive(Default, Clone)]
pub struct Guard {
    direction: usize,
    position_x: usize,
    position_y: usize,
}
pub struct Map {
    up: Vec<usize>,
    right: Vec<usize>,
    down: Vec<usize>,
    left: Vec<usize>,

    guard: Guard,
}

impl Guard {
    fn new(direction: usize, position_x: usize, position_y: usize) -> Self {
        Self {
            direction,
            position_x,
            position_y,
        }
    }
}

impl Map {
    fn solve_part1(&self) -> usize {
        let mut moving_guard = self.guard.clone();
        let mut visited = vec![vec![false; 10]; 10];
        loop {
            match moving_guard.direction {
                UP => {
                    println!(
                        "guard is at {},{} and looks up",
                        moving_guard.position_x, moving_guard.position_y
                    );
                    let max_steps = self.up[moving_guard.position_x];
                    let moving = max_steps - moving_guard.position_x;
                    for i in 0..moving {
                        visited[moving_guard.position_x][moving_guard.position_y - i] = true;
                    }
                    moving_guard.direction = RIGHT;
                    moving_guard.position_y = 9 - max_steps + 1;
                    println!(
                        "no she looks right and is at {}, {}",
                        moving_guard.position_x, moving_guard.position_y
                    );
                }
                RIGHT => {
                    println!(
                        "guard is at {},{} and looks right",
                        moving_guard.position_x, moving_guard.position_y
                    );
                    let max_steps = self.right[moving_guard.position_y] - 1;
                    let moving = max_steps - moving_guard.position_x;
                    for i in 0..moving {
                        visited[moving_guard.position_x + i][moving_guard.position_y] = true;
                        println!(
                            "she visiting {}, {}",
                            moving_guard.position_x + i,
                            moving_guard.position_y
                        );
                    }
                    moving_guard.direction = DOWN;
                    moving_guard.position_x = max_steps;
                    println!(
                        "no she looks down and is at {}, {}",
                        moving_guard.position_x, moving_guard.position_y
                    );
                }
                DOWN => {
                    println!(
                        "guard is at {},{} and looks down",
                        moving_guard.position_x, moving_guard.position_y
                    );
                    let mut max_steps = self.down[moving_guard.position_x];
                    if max_steps == NO_WALL {
                        break;
                    }
                    if max_steps <= moving_guard.position_y {
                        break;
                    }
                    max_steps -= 1;
                    let moving = max_steps - moving_guard.position_y;
                    for i in 0..moving {
                        visited[moving_guard.position_x][moving_guard.position_y + i] = true;
                        println!(
                            "she visiting {}, {}",
                            moving_guard.position_x,
                            moving_guard.position_y + i
                        );
                    }
                    moving_guard.direction = LEFT;
                    moving_guard.position_y = max_steps;
                    println!(
                        "no she looks left and is at {}, {}",
                        moving_guard.position_x, moving_guard.position_y
                    );
                }
                LEFT => {
                    println!(
                        "guard is at {},{} and looks left",
                        moving_guard.position_x, moving_guard.position_y
                    );
                    let mut max_steps = self.left[moving_guard.position_y];
                    if max_steps == NO_WALL {
                        panic!()
                    }
                    let moving = max_steps - (8 - moving_guard.position_x);
                    println!("she can go {moving} steps");
                    let moving = max_steps - moving_guard.position_y;
                    for i in 0..moving {
                        visited[moving_guard.position_x][moving_guard.position_y + i] = true;
                        println!(
                            "she visiting {}, {}",
                            moving_guard.position_x,
                            moving_guard.position_y + i
                        )
                    }
                    println!("max steps would be {max_steps}");
                    moving_guard.direction = UP;
                    moving_guard.position_x = 9 - max_steps;
                    println!(
                        "no she looks up and is at {}, {}",
                        moving_guard.position_x, moving_guard.position_y
                    );
                }
                _ => unreachable!(),
            }
            println!("======== result ");
        }
        visited.into_iter().flatten().filter(|x| *x).count()
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Map;
    type Part1Output = usize;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.solve_part1()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        parse_input(input)
    }
}

fn parse_input(input: &str) -> Map {
    let mut up = vec![NO_WALL; 10];
    let mut right = vec![NO_WALL; 10];
    let mut down = vec![NO_WALL; 10];
    let mut left = vec![NO_WALL; 10];
    let mut guard = Guard::default();

    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    down[column] = row;
                    right[row] = column;
                }
                '^' => guard = Guard::new(UP, column, row),
                '>' => guard = Guard::new(RIGHT, column, row),
                'v' => guard = Guard::new(DOWN, column, row),
                '<' => guard = Guard::new(LEFT, column, row),
                '.' => continue,
                _ => unreachable!(),
            }
        }
    }
    for idx in 0..10 {
        if down[idx] != NO_WALL {
            up[idx] = 9_usize.saturating_sub(down[idx]);
        }
        if right[idx] != NO_WALL {
            left[idx] = 9_usize.saturating_sub(right[idx]);
        }
    }
    Map {
        up,
        right,
        down,
        left,
        guard,
    }
}

#[test]
fn day06() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/demo1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(41, Solver::solve_part1(&parsed));
    assert_eq!(6142, Solver::solve_part2(&parsed));
}
