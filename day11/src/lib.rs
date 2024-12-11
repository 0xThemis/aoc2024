use aoc_traits::AdventOfCodeDay;
use intmap::IntMap;

type Stone = u64;
type Stones = Vec<Stone>;

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Stones;
    type Part1Output = usize;
    type Part2Output = usize;

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        solve_part1(input)
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        solve_part2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        parse_input(input)
    }
}

fn try_split(stone: Stone) -> Option<(Stone, Stone)> {
    let amont_digits = stone.ilog10() + 1;
    if amont_digits % 2 == 0 {
        let split = 10_u64.pow(amont_digits / 2);
        let x = stone / split;
        let y = stone % split;
        Some((x, y))
    } else {
        None
    }
}

fn solve_part2(stones: &Stones) -> usize {
    // first level
    let lvl25 = stones
        .iter()
        .map(|stone| blink25(*stone))
        .flatten()
        .collect::<Vec<_>>();
    // remove duplicates
    let mut duplicates = IntMap::new();
    lvl25.iter().for_each(|stone| {
        duplicates.insert(*stone, ());
    });

    let lvl50 = duplicates
        .keys()
        .into_iter()
        .map(|stone| (stone, blink25(stone)))
        .collect::<Vec<_>>();
    let mut duplicates = IntMap::new();

    lvl50.iter().for_each(|(_, vec)| {
        vec.iter().for_each(|stone| {
            duplicates.insert(*stone, ());
        })
    });

    let mut already_computed = IntMap::new();
    duplicates.keys().into_iter().for_each(|stone| {
        already_computed.insert(stone, blink25(stone).len());
    });

    let mut already_computed_lvl50 = IntMap::new();
    for (lvl50_stone, vec) in lvl50 {
        let mut sum = 0;
        for stone in vec {
            sum += already_computed.get(stone).unwrap();
        }
        already_computed_lvl50.insert(lvl50_stone, sum);
    }
    lvl25
        .into_iter()
        .map(|stone| already_computed_lvl50.get(stone).unwrap())
        .sum()
}

fn blink25(stone: Stone) -> Vec<Stone> {
    let mut next_stones = Stones::with_capacity(2_usize.pow(32));
    let mut current_stones = vec![stone];
    next_stones.reserve(2_usize.pow(32));
    for _ in 0..25 {
        for stone in std::mem::take(&mut current_stones) {
            if stone == 0 {
                next_stones.push(1);
                continue;
            }
            if let Some((x, y)) = try_split(stone) {
                next_stones.push(x);
                next_stones.push(y);
            } else {
                next_stones.push(stone * 2024);
            }
        }
        std::mem::swap(&mut current_stones, &mut next_stones);
    }
    current_stones
}

fn solve_part1(stones: &Stones) -> usize {
    stones.iter().map(|stone| blink25(*stone).len()).sum()
}

fn parse_input(input: &str) -> Stones {
    input
        .split_whitespace()
        .map(|x| x.parse::<Stone>().unwrap())
        .collect()
}

#[test]
fn day11() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());
    assert_eq!(183248, Solver::solve_part1(&parsed));
    let time = std::time::Instant::now();
    assert_eq!(218811774248729, Solver::solve_part2(&parsed));
    let elapsed = time.elapsed();
    println!(
        "took {}.{} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}
