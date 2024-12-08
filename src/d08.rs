use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    row: i16,
    col: i16,
}

struct World {
    width: i16,
    height: i16,
    antennas: Vec<(char, Vec<Pos>)>,
}

impl World {
    fn contains(&self, p: Pos) -> bool {
        p.col >= 0 && p.row >= 0 && p.col < self.width && p.row < self.height
    }
}

fn find_freq(freq: char, antennas: &[(char, Vec<Pos>)]) -> Vec<Pos> {
    antennas
        .iter()
        .find_map(|(f, pp)| if freq == *f { Some(pp.clone()) } else { None })
        .unwrap_or(Vec::new())
}

fn merge(a: Vec<(char, Vec<Pos>)>, b: Vec<(char, Vec<Pos>)>) -> Vec<(char, Vec<Pos>)> {
    let freqs = a
        .iter()
        .map(|(f, _)| *f)
        .chain(b.iter().map(|(f, _)| *f))
        .collect::<HashSet<char>>();

    freqs
        .into_iter()
        .map(|freq| {
            (
                freq,
                find_freq(freq, &a)
                    .into_iter()
                    .chain(find_freq(freq, &b).into_iter())
                    .collect(),
            )
        })
        .collect()
}

fn load(input: &mut dyn Read) -> World {
    let (height, width, antennas) = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .fold(
            (0, 0, Vec::new()),
            |(height, width, antennas), (row, line)| {
                let row_antennas =
                    line.chars()
                        .enumerate()
                        .fold(Vec::new(), |row_antennas, (col, c)| match c {
                            '.' => row_antennas,
                            c => merge(
                                row_antennas,
                                vec![(
                                    c,
                                    vec![Pos {
                                        row: row as i16,
                                        col: col as i16,
                                    }],
                                )],
                            ),
                        });

                (
                    height + 1,
                    width.max(line.len() as i16),
                    merge(antennas, row_antennas),
                )
            },
        );
    World {
        width,
        height,
        antennas,
    }
}

fn antinodes2(
    Pos {
        row: row0,
        col: col0,
    }: Pos,
    Pos {
        row: row1,
        col: col1,
    }: Pos,
) -> [Pos; 2] {
    let row_diff = row1 - row0;
    let col_diff = col1 - col0;

    [
        Pos {
            row: row1 + row_diff,
            col: col1 + col_diff,
        },
        Pos {
            row: row0 - row_diff,
            col: col0 - col_diff,
        },
    ]
}

fn antinodes(antennas: &[Pos]) -> Vec<Pos> {
    antennas
        .iter()
        .copied()
        .map(|antenna0| {
            antennas
                .iter()
                .copied()
                .filter_map(|antenna1| {
                    if antenna0 == antenna1 {
                        None
                    } else {
                        Some((antenna0, antenna1))
                    }
                })
                .collect::<Vec<(Pos, Pos)>>()
        })
        .fold(Vec::new(), |mut a, mut b| {
            a.append(&mut b);
            a
        })
        .into_iter()
        .flat_map(|(a0, a1)| antinodes2(a0, a1))
        .collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let w = load(input);

    let antinodes = w
        .antennas
        .iter()
        .flat_map(|(_, antennas)| antinodes(antennas))
        .filter(|p| w.contains(*p))
        .collect::<HashSet<Pos>>();
    antinodes.len() as u32
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
    fn test_load_example() {
        let mut f = File::open("input/d08-e.txt").unwrap();
        let world = load(&mut f);
        assert_eq!(world.width, 12);
        assert_eq!(world.height, 12);
        assert_eq!(world.antennas.len(), 2);
        assert_eq!(
            world.antennas.iter().map(|(_, aa)| aa.len()).max(),
            Some(4usize)
        );
    }

    #[test]
    fn test_load_full() {
        let mut f = File::open("input/d08-f.txt").unwrap();
        let world = load(&mut f);
        assert_eq!(world.width, 50);
        assert_eq!(world.height, 50);
        assert_eq!(world.antennas.len(), 62);
        assert_eq!(
            world.antennas.iter().map(|(_, aa)| aa.len()).max(),
            Some(4usize)
        );
    }

    #[test]
    fn test_antinodes2() {
        assert_eq!(
            antinodes2(Pos { row: 2, col: 4 }, Pos { row: 2, col: 6 })
                .into_iter()
                .collect::<HashSet<Pos>>(),
            HashSet::from([Pos { row: 2, col: 2 }, Pos { row: 2, col: 8 }])
        );
        assert_eq!(
            antinodes2(Pos { row: 4, col: 2 }, Pos { row: 7, col: 2 })
                .into_iter()
                .collect::<HashSet<Pos>>(),
            HashSet::from([Pos { row: 1, col: 2 }, Pos { row: 10, col: 2 }])
        );
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d08-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d08-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 413);
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
