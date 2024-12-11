use std::collections::HashMap;
use std::io::Read;
use std::iter::repeat;
use std::ops::AddAssign;

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

fn fragment(mut disk: Vec<Option<u16>>) -> Vec<Option<u16>> {
    let frees = disk
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, block)| block.is_none())
        .map(|(pos, _)| pos);
    let to_move = disk
        .iter()
        .copied()
        .enumerate()
        .rev()
        .filter(|(_, block)| block.is_some())
        .map(|(pos, _)| pos);

    for (a, b) in frees
        .zip(to_move)
        .filter(|(a, b)| a < b)
        .collect::<Vec<(usize, usize)>>()
    {
        disk.swap(a, b);
    }

    disk
}

fn first_free(disk: &Vec<Option<u16>>, min_size: usize) -> Option<(usize, usize)> {
    disk.iter()
        .copied()
        .enumerate()
        .scan(None, |current, (pos, block)| {
            match (*current, block) {
                (None, None) => {
                    *current = Some((pos, 1usize));
                }
                (Some((pos, len)), None) => {
                    *current = Some((pos, len + 1));
                }
                (_, Some(_)) => {
                    *current = None;
                }
            }
            Some(current.unwrap_or((0, 0)))
        })
        .find(|(_, len)| *len >= min_size)
}

fn file_map(disk: &Vec<Option<u16>>) -> HashMap<u16, (usize, usize)> {
    disk.iter()
        .copied()
        .enumerate()
        .fold(HashMap::new(), |mut m, (pos, block)| {
            let file_id = match block {
                Some(i) => i,
                None => return m,
            };

            match m.get_mut(&file_id) {
                Some((file_start, file_len)) => {
                    assert!(pos == *file_start + *file_len);
                    file_len.add_assign(1);
                }
                None => {
                    m.insert(file_id, (pos, 1));
                }
            }

            m
        })
}

fn defragment(mut disk: Vec<Option<u16>>) -> Vec<Option<u16>> {
    let mut fmap = file_map(&disk);

    for file_id in (0..=fmap.keys().copied().max().unwrap()).rev() {
        let (file_pos, file_len) = fmap.get(&file_id).unwrap().clone();
        let free_pos = first_free(&disk, file_len).map(|(pos, _)| pos);

        let free_pos = match free_pos {
            Some(p) => {
                if p > file_pos {
                    continue;
                } else {
                    p
                }
            }
            None => {
                continue;
            }
        };

        for n in 0..file_len {
            disk.swap(free_pos + n, file_pos + n);
        }

        fmap.insert(file_id, (free_pos, file_len));
    }

    disk
}

fn part1(input: &mut dyn Read) -> u64 {
    fragment(load(input))
        .into_iter()
        .enumerate()
        .map(|(pos, id)| id.map(|n| n as u64 * pos as u64).unwrap_or(0))
        .sum()
}

fn part2(input: &mut dyn Read) -> u64 {
    defragment(load(input))
        .into_iter()
        .enumerate()
        .map(|(pos, id)| id.map(|n| n as u64 * pos as u64).unwrap_or(0))
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
    fn test_file_map() {
        let mut f = File::open("input/d09-e.txt").unwrap();
        let disk = load(&mut f);
        let fmap = file_map(&disk);

        let mut fmap_sorted = fmap.into_iter().collect::<Vec<(u16, (usize, usize))>>();
        fmap_sorted.sort_by_key(|(k, _)| *k);

        assert_eq!(
            fmap_sorted,
            vec![
                (0, (0, 2)),
                (1, (5, 3)),
                (2, (11, 1)),
                (3, (15, 3)),
                (4, (19, 2)),
                (5, (22, 4)),
                (6, (27, 4)),
                (7, (32, 3)),
                (8, (36, 4)),
                (9, (40, 2))
            ]
        );
    }

    #[test]
    fn test_first_free() {
        let mut f = File::open("input/d09-e.txt").unwrap();
        let disk = load(&mut f);

        assert_eq!(Some((2, 1)), first_free(&disk, 1));
        assert_eq!(Some((2, 2)), first_free(&disk, 2));
        assert_eq!(Some((2, 3)), first_free(&disk, 3));
        assert_eq!(None, first_free(&disk, 4));
    }

    #[test]
    fn test_defragment() {
        let mut f = File::open("input/d09-e.txt").unwrap();
        let disk = defragment(load(&mut f));

        assert_eq!(
            String::from_utf8(
                disk.into_iter()
                    .map(|b| b.map(|n| b'0' + n as u8).unwrap_or(b'.'))
                    .collect::<Vec<u8>>()
            )
            .unwrap(),
            String::from("00992111777.44.333....5555.6666.....8888..")
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
        let mut f = File::open("input/d09-e.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d09-f.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 6415163624282);
    }
}
