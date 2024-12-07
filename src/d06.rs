use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::iter::successors;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Place {
    Outside,
    Obstruction,
    Traversable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    col: i16,
    row: i16,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Speed {
    right: i16,
    down: i16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
                    b'#' => Place::Obstruction,
                    _ => Place::Traversable,
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

fn step_candidates(s: Speed) -> [Speed; 4] {
    let s1 = turn(s);
    let s2 = turn(s1);
    let s3 = turn(s2);
    [s, s1, s2, s3]
}

fn step(g: Guard, m: &Map) -> Guard {
    let (spd, pos) = step_candidates(g.spd)
        .into_iter()
        .map(|spd| (spd, forward(g.pos, spd)))
        .find(|(_, pos)| match m.get(*pos) {
            Place::Obstruction => false,
            _ => true,
        })
        .expect("Did not find a suitable direction");

    Guard { spd, pos }
}

fn printout(guards: &[Guard], map: &Map) {
    let guard = guards.last().unwrap();
    let position_set: HashSet<Pos> = HashSet::from_iter(guards.iter().map(|g| g.pos));
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
            } else if position_set.contains(&pos) {
                'X'
            } else {
                match map.get(pos) {
                    Place::Obstruction => '#',
                    Place::Traversable => '.',
                    _ => panic!("Should not print outside the map"),
                }
            };
            print!("{c}")
        }
        println!();
    }
    println!();
}

fn part1(input: &mut dyn Read) -> (u32, Vec<Guard>, Map) {
    let (guard, map) = load(input);

    let guards = successors(Some(guard), |g| Some(step(*g, &map)))
        .take_while(|g| map.get(g.pos) != Place::Outside)
        .collect::<Vec<Guard>>();

    let position_set: HashSet<Pos> = HashSet::from_iter(guards.iter().map(|g| g.pos));

    (position_set.len() as u32, guards, map)
}

fn part2(input: &mut dyn Read) -> u32 {
    use rayon::prelude::*;

    let (_, guards, map) = part1(input);

    let starting_guard = *guards.first().unwrap();

    let candidates = guards
        .into_iter()
        .filter_map(|g| {
            if g.pos == starting_guard.pos {
                None
            } else {
                Some(g.pos)
            }
        })
        .collect::<HashSet<Pos>>()
        .into_iter()
        .collect::<Vec<Pos>>();

    candidates
        .into_par_iter()
        .filter(|extra_obstruction| {
            let mut m2 = map.clone();
            m2.places[extra_obstruction.row as usize * m2.width + extra_obstruction.col as usize] =
                Place::Obstruction;

            let mut guards = HashSet::new();
            for guard in successors(Some(starting_guard), |g| Some(step(*g, &m2)))
                .take_while(|g| map.get(g.pos) != Place::Outside)
            {
                if guards.contains(&guard) {
                    return true;
                }

                guards.insert(guard);
            }

            false
        })
        .count() as u32
}

pub fn run_part1(input: &mut dyn Read) {
    let (visited, guards, map) = part1(input);
    printout(&guards, &map);
    println!("visited: {visited}");
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
            Place::Traversable
        );
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d06-e.txt").unwrap();
        let (result, _, _) = part1(&mut f);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d06-f.txt").unwrap();
        let (result, _, _) = part1(&mut f);
        assert_eq!(result, 5086);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d06-e.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d06-f.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 1770);
    }
}
