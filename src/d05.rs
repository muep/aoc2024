use std::io::{BufRead, BufReader, Read};

struct Print {
    ordering_rules: Vec<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

impl Print {
    fn load(input: &mut dyn Read) -> Print {
        let mut print = BufReader::new(input)
            .lines()
            .map(|l| l.unwrap())
            .fold(
                (
                    Print {
                        ordering_rules: Vec::new(),
                        updates: Vec::new(),
                    },
                    true,
                ),
                |(mut p, collect_rules), l| {
                    if l.is_empty() {
                        (p, false)
                    } else if collect_rules {
                        let (before, after) = {
                            let mut pieces = l.split('|').map(|a| a.parse::<u8>().unwrap());
                            let before = pieces.next().unwrap();
                            let after = pieces.next().unwrap();
                            (before, after)
                        };
                        p.ordering_rules.push((before, after));
                        (p, true)
                    } else {
                        p.updates
                            .push(l.split(',').map(|p| p.parse::<u8>().unwrap()).collect());
                        (p, false)
                    }
                },
            )
            .0;
        print.ordering_rules.sort();
        print
    }
}

fn is_correct_update(rules: &[(u8, u8)], update: &[u8]) -> bool {
    for pair in update.windows(2) {
        let before = pair[0];
        let after = pair[1];
        for (expected_before, expected_after) in rules.iter().copied() {
            if (before, after) == (expected_after, expected_before) {
                return false;
            }
        }
    }

    true
}

fn part1(input: &mut dyn Read) -> u32 {
    let print = Print::load(input);

    print
        .updates
        .iter()
        .filter(|u| is_correct_update(&print.ordering_rules, u.as_ref()))
        .map(|u| u[u.len() / 2] as u32)
        .sum()
}

fn part2(input: &mut dyn Read) -> u32 {
    BufReader::new(input).lines().count() as u32
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
    fn test_load() {
        let mut f = File::open("input/d05-e.txt").unwrap();
        let p = Print::load(&mut f);
        assert_eq!(p.ordering_rules.len(), 21);
        assert_eq!(p.updates.len(), 6);
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d05-e.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 143);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d05-f.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 6267);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d00-e.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 0);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d00-f.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 0);
    }
}
