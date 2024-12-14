use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

struct Map {
    heights: Vec<u8>,
    length: u16,
    width: u16,
}

impl Map {
    fn load(input: &mut dyn Read) -> Map {
        let (length, width, heights) = BufReader::new(input)
            .lines()
            .map(|l| l.unwrap().bytes().map(|b| (b - b'0')).collect::<Vec<u8>>())
            .fold((0, 0, Vec::new()), |(l, w, mut hs), b| {
                let wnext = w.max(b.len() as u16);
                hs.extend(b);
                (l + 1, wnext, hs)
            });

        Map {
            heights,
            length,
            width,
        }
    }

    fn neigh4(&self, place: u16) -> Vec<u16> {
        let mut n = Vec::new();
        let row = place / self.width;
        let col = place % self.width;

        if row != 0 {
            n.push(place - self.width);
        }

        if row < self.length - 1 {
            n.push(place + self.width);
        }

        if col != 0 {
            n.push(place - 1);
        }

        if col < self.width - 1 {
            n.push(place + 1);
        }

        n
    }
}

fn reachable_tops(map: &Map, pos: u16) -> HashSet<u16> {
    let height = map.heights[pos as usize];

    if height >= 9 {
        return HashSet::from([pos]);
    }

    map.neigh4(pos)
        .into_iter()
        .filter(|neighbor| map.heights[*neighbor as usize] == height + 1)
        .flat_map(|neighbor| reachable_tops(map, neighbor))
        .collect()
}

fn distinct_paths(map: &Map, pos: u16) -> u32 {
    let height = map.heights[pos as usize];

    if height >= 9 {
        return 1;
    }

    map.neigh4(pos)
        .into_iter()
        .filter(|neighbor| map.heights[*neighbor as usize] == height + 1)
        .map(|neighbor| distinct_paths(map, neighbor))
        .sum()
}

fn part1(input: &mut dyn Read) -> u32 {
    let map = Map::load(input);

    (0u16..(map.heights.len() as u16))
        .filter(|position| map.heights[*position as usize] == 0)
        .map(|starting_position| reachable_tops(&map, starting_position).len() as u32)
        .sum()
}

fn part2(input: &mut dyn Read) -> u32 {
    let map = Map::load(input);

    (0u16..(map.heights.len() as u16))
        .filter(|position| map.heights[*position as usize] == 0)
        .map(|starting_position| distinct_paths(&map, starting_position))
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
    fn test_part1_example() {
        let mut f = File::open("input/d10-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d10-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 694);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d10-e.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 81);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d10-f.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 1497);
    }
}
