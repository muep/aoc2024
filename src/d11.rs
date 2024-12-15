use std::io::{BufRead, BufReader, Read};

fn without_leading_zeros(s: &str) -> String {
    let try1 = s.trim_start_matches('0');
    if try1.is_empty() {
        String::from("0")
    } else {
        String::from(try1)
    }
}

fn blink(stones: Vec<String>) -> Vec<String> {
    stones
        .into_iter()
        .flat_map(|s| {
            if s == "0" {
                vec![String::from("1")]
            } else if s.len() % 2 == 0 {
                let (s0, s1) = s.split_at(s.len() / 2);
                vec![String::from(s0), without_leading_zeros(s1)]
            } else {
                vec![format!("{}", s.parse::<u64>().unwrap() * 2024)]
            }
        })
        .collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let stones = BufReader::new(input)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    (0..25).fold(stones, |stones, _| blink(stones)).len() as u32
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
    fn test_part1_example() {
        let mut f = File::open("input/d11-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d11-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 217812);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d00-e.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d00-f.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 0);
    }
}
