use aoc_traits::AdventOfCodeDay;

pub struct Grid {
    map: Vec<Vec<(usize, usize)>>,
    width: usize,
    height: usize,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Grid;
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

impl Grid {
    fn new() -> Self {
        Self {
            map: vec![vec![]; 123],
            width: 0,
            height: 0,
        }
    }

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    fn add_antenna(&mut self, key: char, x: usize, y: usize) {
        let key = key as usize;
        let coords = self.map.get_mut(key).unwrap();
        coords.push((x, y));
    }

    fn solve_part1(&self) -> usize {
        let mut frequencies = vec![vec![false; self.width + 1]; self.height + 1];
        for coords in self.map.iter() {
            for i in 0..coords.len() {
                for j in i + 1..coords.len() {
                    let (first_x, first_y) = coords[i];
                    let (second_x, second_y) = coords[j];
                    let diff_x = second_x - first_x;
                    if first_y < second_y {
                        let diff_y = second_y - first_y;
                        if diff_x <= first_x && diff_y <= first_y {
                            frequencies[first_x - diff_x][first_y - diff_y] = true;
                        }
                        if second_x + diff_x <= self.width && second_y + diff_y <= self.height {
                            frequencies[second_x + diff_x][second_y + diff_y] = true;
                        }
                    } else {
                        let diff_y = first_y - second_y;
                        if diff_x <= first_x && first_y + diff_y <= self.height {
                            frequencies[first_x - diff_x][first_y + diff_y] = true;
                        }
                        if second_x + diff_x <= self.width && diff_y <= second_y {
                            frequencies[second_x + diff_x][second_y - diff_y] = true;
                        }
                    }
                }
            }
        }
        frequencies.into_iter().flatten().filter(|x| *x).count()
    }

    fn solve_part2(&self) -> usize {
        let mut frequencies = vec![vec![false; self.width + 1]; self.height + 1];
        for coords in self.map.iter() {
            for i in 0..coords.len() {
                for j in i + 1..coords.len() {
                    let (first_x, first_y) = coords[i];
                    let (second_x, second_y) = coords[j];
                    let diff_x = second_x - first_x;
                    if first_y < second_y {
                        let diff_y = second_y - first_y;
                        let mut counter = 0;
                        loop {
                            if diff_x * counter <= first_x && diff_y * counter <= first_y {
                                frequencies[first_x - diff_x * counter]
                                    [first_y - diff_y * counter] = true;
                            } else {
                                break;
                            }
                            counter += 1;
                        }
                        counter = 0;
                        loop {
                            if second_x + diff_x * counter <= self.width
                                && second_y + diff_y * counter <= self.height
                            {
                                let x_offset = second_x + diff_x * counter;
                                let y_offset = second_y + diff_y * counter;
                                frequencies[x_offset][y_offset] = true;
                            } else {
                                break;
                            }
                            counter += 1;
                        }
                    } else {
                        let diff_y = first_y - second_y;
                        let mut counter = 0;
                        loop {
                            if diff_x * counter <= first_x
                                && first_y + diff_y * counter <= self.height
                            {
                                frequencies[first_x - diff_x * counter]
                                    [first_y + diff_y * counter] = true;
                            } else {
                                break;
                            }
                            counter += 1;
                        }
                        counter = 0;
                        loop {
                            if second_x + diff_x * counter <= self.width
                                && diff_y * counter <= second_y
                            {
                                frequencies[second_x + diff_x * counter]
                                    [second_y - diff_y * counter] = true;
                            } else {
                                break;
                            }
                            counter += 1;
                        }
                    }
                }
            }
        }
        frequencies.into_iter().flatten().filter(|x| *x).count()
    }
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid::new();
    for (row, line) in input.lines().enumerate() {
        grid.set_height(row);
        for (col, char) in line.chars().enumerate() {
            grid.set_width(col);
            if char.is_ascii_alphanumeric() {
                grid.add_antenna(char, row, col);
            }
        }
    }
    grid
}

#[test]
fn day08() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(220, Solver::solve_part1(&parsed));
    assert_eq!(813, Solver::solve_part2(&parsed));
}
