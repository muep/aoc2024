use std::io::{BufRead, BufReader, Read};
use std::iter::{repeat, successors};

enum LoadNext {
    ContentForFile(u16),
    SpaceAfterFile(u16),
}

fn load(input: &mut dyn Read) -> Vec<Option<u16>> {
    input
        .bytes()
        .filter_map(|b| match b {
            Ok(n) => {
                if n.is_ascii_digit() {
                    Some((n - b'0') as u16)
                } else {
                    None
                }
            }
            _ => None,
        })
        .fold(
            (LoadNext::ContentForFile(0), Vec::new()),
            |(next, mut buf), num_items| match next {
                LoadNext::ContentForFile(id) => {
                    buf.extend(repeat(Some(id)).take(num_items as usize));
                    (LoadNext::SpaceAfterFile(id), buf)
                }
                LoadNext::SpaceAfterFile(id) => {
                    buf.extend(repeat(None).take(num_items as usize));
                    (LoadNext::ContentForFile(id + 1), buf)
                }
            },
        )
        .1
}

fn fragment(mut disk: Vec<Option<u16>>) -> (bool, Vec<Option<u16>>) {
    let next_free = disk
        .iter()
        .copied()
        .enumerate()
        .find(|(_, slot)| slot.is_none())
        .map(|(pos, _)| pos)
        .unwrap_or(0);

    let next_to_move = disk
        .iter()
        .copied()
        .enumerate()
        .rfind(|(_, slot)| slot.is_some())
        .map(|(pos, _)| pos)
        .unwrap_or(0);

    if next_free >= next_to_move {
        return (false, disk);
    }

    disk.swap(next_to_move, next_free);
    (true, disk)
}

fn part1(input: &mut dyn Read) -> u64 {
    successors(Some(load(input)), |disk| {
        let (did_change, disk) = fragment(disk.clone());
        if did_change {
            Some(disk)
        } else {
            None
        }
    })
    .last()
    .unwrap()
    .into_iter()
    .enumerate()
    .map(|(pos, id)| id.map(|n| n as u64 * pos as u64).unwrap_or(0))
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
    fn test_parse_digit() {
        let digit = b'4';
        let num = digit - b'0';

        assert_eq!(num, 4);
    }

    #[test]
    fn test_load_example() {
        let mut f = File::open("input/d09-e.txt").unwrap();
        let disk = load(&mut f);
        assert_eq!(
            disk,
            "00...111...2...333.44.5555.6666.777.888899"
                .bytes()
                .map(|c| match c {
                    b'.' => None,
                    n => Some(n as u16 - b'0' as u16),
                })
                .collect::<Vec<Option<u16>>>()
        );
    }

    #[test]
    fn test_load_full() {
        let mut f = File::open("input/d09-f.txt").unwrap();
        let disk = load(&mut f);
        assert_eq!(disk.len(), 95177);
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d09-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d09-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 6385338159127);
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
