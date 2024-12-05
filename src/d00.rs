use std::io::{BufRead, BufReader, Read};

fn part1(input: &mut dyn Read) -> u32 {
    BufReader::new(input).lines().count() as u32
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
        let mut f = File::open("input/d00-e.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 0);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d00-f.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 0);
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
