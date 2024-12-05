use aoc_traits::AdventOfCodeDay;
use intmap::IntMap;

type Update = Vec<u32>;

struct OrderingRules {
    rules: IntMap<()>,
}

pub struct PrintHandout {
    ordering_rules: OrderingRules,
    updates: Vec<Update>,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = PrintHandout;
    type Part1Output = u32;
    type Part2Output = u32;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut result = 0;
        for update in input.updates.iter() {
            let update_len = update.len();
            let mut is_wrong = false;
            for i in 0..update_len {
                for j in i + 1..update_len {
                    if input.ordering_rules.is_wrong(update[i], update[j]) {
                        is_wrong = true;
                        break;
                    }
                }
            }
            if !is_wrong {
                result += update[update.len().div_ceil(2) - 1];
            }
        }
        result
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut result = 0;
        for update in input.updates.iter() {
            let mut cloned_update = update.clone();
            let mut is_wrong = false;
            let mut is_this_wrong = false;
            let mut i = 0;
            loop {
                if i == update.len() {
                    break;
                }
                for j in i + 1..update.len() {
                    if input
                        .ordering_rules
                        .is_wrong(cloned_update[i], cloned_update[j])
                    {
                        let tmp = cloned_update[i];
                        cloned_update[i] = cloned_update[j];
                        cloned_update[j] = tmp;
                        is_wrong = true;
                        is_this_wrong = true;
                        i = i.saturating_sub(1);
                        break;
                    }
                }
                if is_this_wrong {
                    is_this_wrong = false;
                    continue;
                }
                i += 1;
            }
            if is_wrong {
                result += cloned_update[update.len().div_ceil(2) - 1];
            }
        }
        result
    }

    fn parse_input<'a>(input: &'a str) -> Self::ParsedInput<'a> {
        parse_input(input)
    }
}

impl PrintHandout {
    fn new() -> Self {
        Self {
            ordering_rules: OrderingRules::new(),
            updates: Vec::with_capacity(6000),
        }
    }

    fn add_rule(&mut self, left: u32, right: u32) {
        self.ordering_rules.add_rule(left, right)
    }

    fn add_update(&mut self, update: Update) {
        self.updates.push(update);
    }
}

impl OrderingRules {
    fn new() -> Self {
        Self {
            rules: IntMap::new(),
        }
    }
    fn add_rule(&mut self, left: u32, right: u32) {
        let mut key = (right as u64) << 32;
        key |= left as u64;
        self.rules.insert(key, ());
    }

    fn is_wrong(&self, left: u32, right: u32) -> bool {
        let mut key = (left as u64) << 32;
        key |= right as u64;
        self.rules.contains_key(key)
    }
}

fn parse_input(input: &str) -> PrintHandout {
    let mut print_handout = PrintHandout::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut split = line.trim().split('|');
        let left = split.next().unwrap().parse::<u32>().unwrap();
        let right = split.next().unwrap().parse::<u32>().unwrap();
        print_handout.add_rule(left, right)
    }
    while let Some(line) = lines.next() {
        let split = line.split(",");
        print_handout.add_update(
            split
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        )
    }
    print_handout
}

#[test]
fn day04() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(5391, Solver::solve_part1(&parsed));
    assert_eq!(6142, Solver::solve_part2(&parsed));
}
