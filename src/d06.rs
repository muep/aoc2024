use std::io::{BufRead, BufReader, Read};
use crate::d06::Place::{Obstruction, Traversable};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Place {
    Obstruction,
    Traversable(u8),
}

struct Guard {
    pos_col: i16,
    pos_row: i16,
    speed_hz: i16,
    speed_vt: i16,
}

struct Map {
    width: usize,
    places: Vec<Place>,
}

impl Map {
    fn load(input: &mut dyn Read) -> (Guard, Map) {
        let (width, places, (pos_col, pos_row, speed_hz, speed_vt)) = BufReader::new(input)
            .lines()
            .map(|s| s.unwrap().bytes().collect::<Vec<u8>>())
            .fold(
                (0, Vec::new(), (0i16, 0i16, 0i16, 1i16)),
                |(width, mut places, g), line| {
                    let width = width.max(line.len());

                    let guard = line
                        .iter()
                        .copied()
                        .enumerate()
                        .find(|(_, b)| b"^v<>".contains(b))
                        .map(|(offset, b)| {
                            let row = places.len() / width;
                            let col = offset;

                            let (speed_vt, speed_hz) = match b {
                                b'^' => (-1, 0),
                                b'v' => (1, 0),
                                b'<' => (0, -1),
                                b'>' => (0, 1),
                                _ => panic!("not expected to happen"),
                            };

                            (col as i16, row as i16, speed_vt, speed_hz)
                        })
                        .unwrap_or(g);

                    places.extend(line.into_iter().map(|b| match b {
                        b'.' => Traversable(0),
                        b'#' => Obstruction,
                        b'^' => Traversable(1),
                        b'v' => Traversable(1),
                        b'<' => Traversable(1),
                        b'>' => Traversable(1),

                        _ => panic!("Unexpected map content {b:?}"),
                    }));
                    (width, places, guard)
                },
            );

        (
            Guard {
                pos_col,
                pos_row,
                speed_hz,
                speed_vt,
            },
            Map { width, places },
        )
    }
}

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
    fn test_load()
    {
        let mut f = File::open("input/d06-e.txt").unwrap();

        let (guard, map) = Map::load(&mut f);

        assert_eq!(map.width, 10);
        assert_eq!(map.places.len() / map.width, 10);
        assert_eq!(guard.pos_col, 4);
        assert_eq!(guard.pos_row, 6);
        assert_eq!(map.places[map.width * guard.pos_row as usize + guard.pos_col as usize], Traversable(1));
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d00-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d00-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 0);
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
