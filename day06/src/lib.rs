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

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
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
}

pub struct Map {
    map: Vec<Vec<Field>>,
    direction: Direction,
    path: Vec<(usize, usize)>,
    x: usize,
    y: usize,
}

impl Map {
    fn solve_part2(&self) -> usize {
        let mut map = self.map.clone();
        let mut first_path = self.path.clone().into_iter();
        // not allowed on first
        first_path.next().unwrap();

        let mut counter = 0;
        let mut already_taken = vec![vec![false; map[0].len()]; map.len()];

        for (x, y) in first_path {
            if already_taken[x][y] {
                continue;
            }
            already_taken[x][y] = true;
            map[x][y] = Field::Obstacle;
            if self.is_loop(&map) {
                counter += 1;
            }
            map[x][y] = Field::Free;
        }
        counter
    }

    fn is_loop(&self, map: &[Vec<Field>]) -> bool {
        let mut visited = vec![vec![vec![false; 4]; map[0].len() + 1]; map.len() + 1];
        let mut x = self.x;
        let mut y = self.y;
        let mut direction = self.direction;
        let is_loop = loop {
            // check if we are on circle
            if visited[x][y][direction as usize] {
                return true;
            }
            visited[x][y][direction as usize] = true;

            match direction {
                Direction::Up => match map[x - 1][y] {
                    Field::Obstacle => {
                        direction = Direction::Right;
                    }
                    Field::Free => x -= 1,
                    Field::Over => break false,
                },
                Direction::Right => match map[x][y + 1] {
                    Field::Obstacle => {
                        direction = Direction::Down;
                    }
                    Field::Free => y += 1,
                    Field::Over => break false,
                },
                Direction::Down => match map[x + 1][y] {
                    Field::Obstacle => {
                        direction = Direction::Left;
                    }
                    Field::Free => x += 1,
                    Field::Over => break false,
                },
                Direction::Left => match map[x][y - 1] {
                    Field::Obstacle => {
                        direction = Direction::Up;
                    }
                    Field::Free => y -= 1,
                    Field::Over => break false,
                },
            }
        };
        is_loop
    }

    fn solve_part1(&self) -> usize {
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        for (x, y) in self.path.iter() {
            visited[*x][*y] = true;
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
        input.solve_part2()
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
    for (row, line) in lines.enumerate() {
        map.push(parse_line(line, &mut guard, row + 1));
    }
    map.push(vec![Field::Over; width]);
    Map {
        path: get_path(guard.x, guard.y, guard.direction, &map),
        map,
        direction: guard.direction,
        x: guard.x,
        y: guard.y,
    }
}

fn get_path(
    mut x: usize,
    mut y: usize,
    mut direction: Direction,
    map: &[Vec<Field>],
) -> Vec<(usize, usize)> {
    let mut path = Vec::with_capacity(map[0].len() * map.len());
    loop {
        path.push((x, y));
        match direction {
            Direction::Up => match map[x - 1][y] {
                Field::Obstacle => {
                    direction = Direction::Right;
                }
                Field::Free => x -= 1,
                Field::Over => break,
            },
            Direction::Right => match map[x][y + 1] {
                Field::Obstacle => {
                    direction = Direction::Down;
                }
                Field::Free => y += 1,
                Field::Over => break,
            },
            Direction::Down => match map[x + 1][y] {
                Field::Obstacle => {
                    direction = Direction::Left;
                }
                Field::Free => x += 1,
                Field::Over => break,
            },
            Direction::Left => match map[x][y - 1] {
                Field::Obstacle => {
                    direction = Direction::Up;
                }
                Field::Free => y -= 1,
                Field::Over => break,
            },
        }
    }
    path
}

#[test]
fn day06() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(4973, Solver::solve_part1(&parsed));
    assert_eq!(1482, Solver::solve_part2(&parsed));
}
