use std::io::{BufRead, BufReader, Read};

enum State {
    Empty,
    One(u32),
    Up(u32),
    Down(u32),
    Unsafe,
}

fn safe_distance(a: u32, b: u32) -> bool {
    const OK_DISTANCES: &[u32] = &[1, 2, 3];
    OK_DISTANCES.contains(&a.abs_diff(b))
}

fn safe_step(lower: u32, higher: u32) -> bool {
    safe_distance(lower, higher) && lower < higher
}

fn safe_report(levels: &[u32]) -> bool {
    match levels
        .iter()
        .copied()
        .fold(State::Empty, |state, level| match state {
            State::Empty => State::One(level),
            State::One(prev) => match (prev < level, safe_distance(prev, level)) {
                (false, true) => State::Down(level),
                (true, true) => State::Up(level),
                _ => State::Unsafe,
            },
            State::Up(prev) => {
                if safe_step(prev, level) {
                    State::Up(level)
                } else {
                    State::Unsafe
                }
            }
            State::Down(prev) => {
                if safe_step(level, prev) {
                    State::Down(level)
                } else {
                    State::Unsafe
                }
            }
            a => a,
        }) {
        State::Up(_) => true,
        State::Down(_) => true,
        _ => false,
    }
}

fn safe_report_v2(levels: &[u32]) -> bool {
    if safe_report(levels) {
        return true;
    }

    (0..levels.len())
        .map(|pos| {
            levels
                .iter()
                .copied()
                .take(pos)
                .chain(levels.iter().copied().skip(pos + 1))
                .collect::<Vec<u32>>()
        })
        .any(|reduced_levels| safe_report(&reduced_levels))
}

fn part(f: fn(&[u32]) -> bool, input: &mut dyn Read) -> u32 {
    BufReader::new(input)
        .lines()
        .map(|l| {
            let levels: Vec<u32> = l
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            if f(&levels) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part1(input: &mut dyn Read) -> u32 {
    part(safe_report, input)
}

fn part2(input: &mut dyn Read) -> u32 {
    part(safe_report_v2, input)
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}
pub fn run_part2(input: &mut dyn Read) {
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_safe_report() {
        assert!(safe_report(&[1, 2, 3, 4, 5]));
        assert!(safe_report(&[5, 4, 3, 2, 1]));
        assert!(safe_report(&[7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_safe_report_v2() {
        assert!(safe_report(&[7, 6, 4, 2, 1]));
        assert!(!safe_report(&[1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d02-e.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 2);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d02-f.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 379);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d02-e.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 4);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d02-f.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 430);
    }
}
