use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};

struct Print {
    ordering_rules: Vec<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

impl Print {
    fn buf(&self) -> Vec<u8> {
        let sz = self.updates.iter().map(|v| v.len()).max().unwrap();
        let mut buf = Vec::with_capacity(sz);
        buf.resize(sz, 0);
        buf
    }

    fn load(input: &mut dyn Read) -> Print {
        let (mut rules, updates, _) = BufReader::new(input).lines().map(|l| l.unwrap()).fold(
            (Vec::new(), Vec::new(), true),
            |(mut rules, mut updates, collect_rules), l| {
                if l.is_empty() {
                    (rules, updates, false)
                } else if collect_rules {
                    let (before, after) = {
                        let mut pieces = l.split('|').map(|a| a.parse::<u8>().unwrap());
                        let before = pieces.next().unwrap();
                        let after = pieces.next().unwrap();
                        (before, after)
                    };
                    rules.push((before, after));
                    (rules, updates, true)
                } else {
                    updates.push(l.split(',').map(|p| p.parse::<u8>().unwrap()).collect());
                    (rules, updates, false)
                }
            },
        );

        rules.sort();
        Print {
            ordering_rules: rules,
            updates: updates,
        }
    }
}

fn sort_by_rules(rules: &[(u8, u8)], buf: &mut [u8]) {
    buf.sort_by(|a, b| {
        if a == b {
            return Ordering::Equal;
        }

        for (before, after) in rules.iter() {
            if (before, after) == (a, b) {
                return Ordering::Less;
            }

            if (before, after) == (b, a) {
                return Ordering::Greater;
            }
        }

        a.cmp(&b)
    })
}

fn is_correct_update(rules: &[(u8, u8)], update: &[u8], buf: &mut [u8]) -> bool {
    buf.copy_from_slice(update);
    sort_by_rules(rules, buf);
    buf == update
}

fn part1(input: &mut dyn Read) -> u32 {
    let print = Print::load(input);
    let mut buf = print.buf();

    print
        .updates
        .iter()
        .filter(|u| is_correct_update(&print.ordering_rules, u.as_ref(), &mut buf[0..u.len()]))
        .map(|u| u[u.len() / 2] as u32)
        .sum()
}

fn part2(input: &mut dyn Read) -> u32 {
    let print = Print::load(input);
    let mut buf = print.buf();

    let Print {
        ordering_rules: rules,
        updates,
    } = print;

    updates
        .into_iter()
        .filter(|u| !is_correct_update(&rules, u.as_ref(), &mut buf[0..u.len()]))
        .map(|mut u| {
            sort_by_rules(&rules, &mut u);
            u[u.len() / 2] as u32
        })
        .sum()
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
        let mut f = File::open("input/d05-e.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 123);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d05-f.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 5184);
    }
}
