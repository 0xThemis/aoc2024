use std::usize;

use aoc_traits::AdventOfCodeDay;

#[derive(Clone, Copy)]
enum Field {
    Obstacle,
    Free,
    Over,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Obstacle => write!(f, "#"),
            Field::Free => write!(f, "."),
            Field::Over => write!(f, "o"),
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Debug, Default, Clone)]
struct Guard {
    direction: Direction,
    x: usize,
    y: usize,
}

impl Guard {
    pub fn new() -> Self {
        Guard::default()
    }

    pub fn set_coords(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    pub fn turn(&mut self) {
        let direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self.direction = direction;
    }

    pub fn what_next_step(&mut self, map: &[Vec<Field>]) -> Field {
        match self.direction {
            Direction::Up => map[self.x - 1][self.y],
            Direction::Right => map[self.x][self.y + 1],
            Direction::Down => map[self.x + 1][self.y],
            Direction::Left => map[self.x][self.y - 1],
        }
    }
    pub fn go(&mut self) {
        match self.direction {
            Direction::Up => self.x -= 1,
            Direction::Right => self.y += 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
        }
    }
}

pub struct Map {
    map: Vec<Vec<Field>>,
    guard: Guard,
}

impl Map {
    fn solve_part1(&self) -> usize {
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        visited[self.guard.x][self.guard.y] = true;
        let mut moving_guard = self.guard.clone();
        loop {
            match moving_guard.what_next_step(&self.map) {
                Field::Obstacle => {
                    visited[moving_guard.x][moving_guard.y] = true;
                    moving_guard.turn();
                }
                Field::Free => visited[moving_guard.x][moving_guard.y] = true,
                Field::Over => break,
            }
            moving_guard.go();
        }
        visited.into_iter().flatten().filter(|x| *x).count() + 1
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

fn parse_line(line: &str, guard: &mut Guard, row: usize) -> Vec<Field> {
    let mut current_row = Vec::with_capacity(line.len());
    current_row.push(Field::Over);
    for (col, char) in line.chars().enumerate() {
        match char {
            '#' => current_row.push(Field::Obstacle),
            '^' => {
                current_row.push(Field::Free);
                guard.set_coords(row + 1, col + 1);
            }
            '.' => current_row.push(Field::Free),
            _ => unreachable!(),
        }
    }
    current_row.push(Field::Over);
    current_row
}

fn parse_input(input: &str) -> Map {
    let mut map = vec![];
    let mut guard = Guard::new();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let width = first_line.len();
    map.push(vec![Field::Over; width]);
    map.push(parse_line(first_line, &mut guard, 0));
    for (row, line) in input.lines().enumerate() {
        map.push(parse_line(line, &mut guard, row));
    }
    map.push(vec![Field::Over; width]);
    Map { guard, map }
}

#[test]
fn day06() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(4973, Solver::solve_part1(&parsed));
    //assert_eq!(6142, Solver::solve_part2(&parsed));
}
