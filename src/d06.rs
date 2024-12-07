use std::io::{BufRead, BufReader, Read};
use std::iter::successors;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Place {
    Outside,
    Obstruction,
    Traversable(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    col: i16,
    row: i16,
}
#[derive(Clone, Copy, Debug)]
struct Speed {
    right: i16,
    down: i16,
}

#[derive(Clone, Copy, Debug)]
struct Guard {
    pos: Pos,
    spd: Speed,
}

#[derive(Clone, Debug)]
struct Map {
    width: usize,
    places: Vec<Place>,
}

impl Map {
    fn get(&self, Pos { col, row }: Pos) -> Place {
        let height = self.places.len() / self.width;

        if col < 0 || col >= self.width as i16 || row < 0 || row >= height as i16 {
            return Place::Outside;
        }

        self.places[row as usize * self.width + col as usize]
    }

    fn traverse(&mut self, Pos { col, row }: Pos) {
        let height = self.places.len() / self.width;

        if col < 0 || col >= self.width as i16 || row < 0 || row >= height as i16 {
            return;
        }

        if let Place::Traversable(n) = self.places[row as usize * self.width + col as usize] {
            self.places[row as usize * self.width + col as usize] =
                Place::Traversable(n.saturating_add(1));
        }
    }
}

fn load(input: &mut dyn Read) -> (Guard, Map) {
    let (width, places, guard) = BufReader::new(input)
        .lines()
        .map(|s| s.unwrap().bytes().collect::<Vec<u8>>())
        .fold(
            (
                0,
                Vec::new(),
                Guard {
                    pos: Pos { col: 0, row: 0 },
                    spd: Speed { right: 1, down: 0 },
                },
            ),
            |(width, mut places, g), line| {
                let width = width.max(line.len());

                let guard = line
                    .iter()
                    .copied()
                    .enumerate()
                    .find(|(_, b)| b"^v<>".contains(b))
                    .map(|(offset, b)| {
                        let row = (places.len() / width) as i16;
                        let col = offset as i16;

                        let (down, right) = match b {
                            b'^' => (-1, 0),
                            b'v' => (1, 0),
                            b'<' => (0, -1),
                            b'>' => (0, 1),
                            _ => panic!("not expected to happen"),
                        };

                        Guard {
                            pos: Pos { row, col },
                            spd: Speed { down, right },
                        }
                    })
                    .unwrap_or(g);

                places.extend(line.into_iter().map(|b| match b {
                    b'.' => Place::Traversable(0),
                    b'#' => Place::Obstruction,
                    b'^' => Place::Traversable(1),
                    b'v' => Place::Traversable(1),
                    b'<' => Place::Traversable(1),
                    b'>' => Place::Traversable(1),

                    _ => panic!("Unexpected map content {b:?}"),
                }));
                (width, places, guard)
            },
        );

    (guard, Map { width, places })
}

fn turn(Speed { down, right }: Speed) -> Speed {
    Speed {
        down: right,
        right: -down,
    }
}

fn forward(Pos { col, row }: Pos, Speed { down, right }: Speed) -> Pos {
    Pos {
        col: col + right,
        row: row + down,
    }
}

fn step(g: Guard, mut m: Map) -> (Guard, Map) {
    let (spd, pos) = successors(Some(g.spd), |s| Some(turn(*s)))
        .take(4)
        .map(|spd| (spd, forward(g.pos, spd)))
        .find(|(_, pos)| match m.get(*pos) {
            Place::Obstruction => false,
            _ => true,
        })
        .expect("Did not find a suitable direction");

    m.traverse(pos);

    (Guard { spd, pos }, m)
}

fn printout(guard: Guard, map: &Map) {
    let Speed { down, right } = guard.spd;
    let height = map.places.len() / map.width;
    for row in 0..height as i16 {
        for col in 0..map.width as i16 {
            let pos = Pos { col, row };

            let c = if pos == guard.pos {
                match (down, right) {
                    (0, 1) => '>',
                    (0, -1) => '<',
                    (1, 0) => 'v',
                    (-1, 0) => '^',
                    _ => '@',
                }
            } else {
                match map.get(pos) {
                    Place::Obstruction => '#',
                    Place::Traversable(0) => '.',
                    Place::Traversable(1) => '-',
                    Place::Traversable(2) => '+',
                    Place::Traversable(_) => '*',
                    _ => panic!("Should not print outside the map"),
                }
            };
            print!("{c}")
        }
        println!();
    }
    println!();
}

fn stat(map: &Map) -> (usize, usize, usize) {
    map.places.iter().copied().fold(
        (0usize, 0usize, 0usize),
        |(obstructions, unvisited, visited), p| match p {
            Place::Obstruction => (obstructions + 1, unvisited, visited),
            Place::Traversable(0) => (obstructions, unvisited + 1, visited),
            Place::Traversable(_) => (obstructions, unvisited, visited + 1),
            _ => panic!("not expected"),
        },
    )
}

fn part1(input: &mut dyn Read) -> u32 {
    let (guard, map) = successors(Some(load(input)), |(g, m)| Some(step(*g, m.clone())))
        .take_while(|(g, m)| m.get(g.pos) != Place::Outside)
        .last()
        .unwrap();

    printout(guard, &map);
    let (obsctuctions, unvisited, visited) = stat(&map);
    println!("obstructed: {obsctuctions}\nunvisited: {unvisited}\nvisited: {visited}");
    visited as u32
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
        let mut f = File::open("input/d06-e.txt").unwrap();

        let (guard, map) = load(&mut f);

        assert_eq!(map.width, 10);
        assert_eq!(map.places.len() / map.width, 10);
        assert_eq!(guard.pos.col, 4);
        assert_eq!(guard.pos.row, 6);
        assert_eq!(
            map.places[map.width * guard.pos.row as usize + guard.pos.col as usize],
            Place::Traversable(1)
        );
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d06-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d06-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 5086);
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
