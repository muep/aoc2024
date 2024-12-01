use std::io::{BufRead, BufReader, Read};

fn read_line(line: &str) -> (u32, u32, u32) {
    let mut pieces = line
        .split("x")
        .map(|a| a.parse::<u32>().unwrap())
        .into_iter();
    let a = pieces.next().unwrap();
    let b = pieces.next().unwrap();
    let c = pieces.next().unwrap();
    (a, b, c)
}

fn wrapping_amount((l, w, h): (u32, u32, u32)) -> u32 {
    let s0 = l * w;
    let s1 = w * h;
    let s2 = h * l;
    2 * s0 + 2 * s1 + 2 * s2 + [s0, s1, s2].into_iter().min().unwrap()
}

fn part1(input: &mut dyn Read) -> u32 {
    BufReader::new(input)
        .lines()
        .map(|l| read_line(&l.unwrap()))
        .map(wrapping_amount)
        .sum()
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    #[test]
    fn test_read_line() {
        assert_eq!(read_line("13x5x19"), (13, 5, 19));
    }

    #[test]
    fn test_wrap_amount() {
        assert_eq!(wrapping_amount((2, 3, 4)), 58);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d00.txt").unwrap();
        assert_eq!(part1(&mut f), 1588178);
    }
}
