use aoc_traits::AdventOfCodeDay;

#[derive(Clone, Debug)]
pub enum DiskSpace {
    File(usize, usize),
    Free(usize),
}

pub struct Disk {
    part1: Vec<DiskSpace>,
    part2: Vec<DiskSpace>,
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Disk;
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

fn solve_part2(disk: &Disk) -> usize {
    let mut for_expansion = disk.part2.clone();
    loop {
        if !move_around(&mut for_expansion) {
            break;
        }
    }
    let mut expanded = Vec::with_capacity(disk.part1.len());
    for ele in for_expansion {
        match ele {
            DiskSpace::File(id, len) => expanded.extend(vec![DiskSpace::File(id, 1); len]),
            DiskSpace::Free(len) => expanded.extend(vec![DiskSpace::Free(1); len]),
        }
    }

    expanded
        .into_iter()
        .enumerate()
        .map(|(idx, space)| match space {
            DiskSpace::File(id, _) => id * idx,
            DiskSpace::Free(_) => 0,
        })
        .sum()
}

fn move_around(disk: &mut Vec<DiskSpace>) -> bool {
    let cloned = disk.clone();
    for (rev_idx, space) in cloned.iter().enumerate().rev() {
        match space {
            DiskSpace::File(id, len) => {
                //check if we can move
                for (idx, x) in cloned.iter().enumerate() {
                    if idx >= rev_idx {
                        break;
                    }
                    match x {
                        DiskSpace::File(_, _) => continue,
                        DiskSpace::Free(free) => {
                            if free >= len {
                                let remaining_size = free - len;
                                disk[idx] = DiskSpace::File(*id, *len);
                                disk[rev_idx] = DiskSpace::Free(*len);
                                disk.insert(idx + 1, DiskSpace::Free(remaining_size));
                                return true;
                            }
                        }
                    }
                }
            }
            DiskSpace::Free(_) => continue,
        }
    }
    false
}

fn solve_part1(disk: &Disk) -> usize {
    let mut disk = disk.part1.clone();
    let mut rev = disk.len();
    let mut free_space = 0;
    loop {
        rev -= 1;
        match &disk[rev] {
            s @ DiskSpace::File(_, _) => {
                // find next free
                loop {
                    match disk[free_space] {
                        DiskSpace::File(_, _) => {
                            free_space += 1;
                            if rev < free_space {
                                break;
                            }
                        }
                        DiskSpace::Free(_) => {
                            //swap
                            disk[free_space] = s.clone();
                            disk[rev] = DiskSpace::Free(0);
                            break;
                        }
                    }
                }
            }
            DiskSpace::Free(_) => continue,
        }
        if rev < free_space {
            break;
        }
    }
    disk.into_iter()
        .take(rev + 1)
        .enumerate()
        .map(|(idx, space)| match space {
            DiskSpace::File(id, _) => id * idx,
            DiskSpace::Free(_) => unreachable!(),
        })
        .sum()
}

fn parse_input(input: &str) -> Disk {
    let mut disk1 = Vec::with_capacity(65536);
    let mut disk2 = Vec::with_capacity(65536);
    let mut is_file = true;
    let mut file_id = 0;
    for char in input.chars() {
        let indicator = char.to_digit(10).unwrap() as usize;
        let to_push = if is_file {
            let space = DiskSpace::File(file_id, indicator);
            file_id += 1;
            space
        } else {
            DiskSpace::Free(indicator)
        };
        disk1.extend(vec![to_push.clone(); indicator]);
        disk2.push(to_push);
        is_file = !is_file;
    }
    Disk {
        part1: disk1,
        part2: disk2,
    }
}

#[test]
fn day09() {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let input = std::fs::read_to_string(format!("{root}/inputs/puzzle1.txt")).unwrap();
    let parsed = Solver::parse_input(input.trim());

    assert_eq!(6283404590840, Solver::solve_part1(&parsed));
    assert_eq!(6304576012713, Solver::solve_part2(&parsed));
}
