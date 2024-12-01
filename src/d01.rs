use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

fn read_line(line: &str) -> (u32, u32) {
    let mut pieces = line.split_whitespace();
    let l = pieces.next().unwrap().parse::<u32>().unwrap();
    let r = pieces.next().unwrap().parse::<u32>().unwrap();
    (l, r)
}

fn load(input: &mut dyn Read) -> (Vec<u32>, Vec<u32>) {
    BufReader::new(input)
        .lines()
        .map(|l| read_line(&l.unwrap().to_string()))
        .unzip()
}

fn part1(input: &mut dyn Read) -> u32 {
    let (left, right) = {
        let (mut left, mut right) = load(input);

        left.sort();
        right.sort();

        (left, right)
    };

    left.into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn counts(nums: &[u32]) -> HashMap<u32, u32> {
    nums.into_iter().fold(HashMap::new(), |mut cts, n| {
        let ct = cts.get(n).unwrap_or(&0) + 1;
        cts.insert(*n, ct);
        cts
    })
}

fn part2(input: &mut dyn Read) -> u32 {
    let (left, right) = {
        let (left, right) = load(input);

        (counts(&left), counts(&right))
    };

    left.into_iter()
        .map(|(num, ct)| num * ct * right.get(&num).unwrap_or(&0))
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
    fn test_read_line() {
        let line = "3   4";
        let (left, right) = read_line(line);
        assert_eq!(left, 3);
        assert_eq!(right, 4);
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d01-e.txt").unwrap();
        let dist = part1(&mut f);
        assert_eq!(dist, 11);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d01-f.txt").unwrap();
        let dist = part1(&mut f);
        assert_eq!(dist, 1197984);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d01-e.txt").unwrap();
        let dist = part2(&mut f);
        assert_eq!(dist, 31);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d01-f.txt").unwrap();
        let dist = part2(&mut f);
        assert_eq!(dist, 23387399);
    }
}
