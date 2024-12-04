use std::io::{BufRead, BufReader, Read};

struct WordSearch {
    width: usize,
    buf: Vec<char>,
}

impl WordSearch {
    fn height(&self) -> usize {
        self.buf.len() / self.width
    }
    fn load(input: &mut dyn Read) -> WordSearch {
        let (pitch, buf) = {
            let (pitch, buf) = BufReader::new(input).lines().map(|a| a.unwrap()).fold(
                (None, String::new()),
                |(pitch, mut buf), line| {
                    buf.push_str(&line);

                    (Some(pitch.unwrap_or(line.len())), buf)
                },
            );
            (pitch.unwrap(), buf)
        };

        WordSearch {
            width: pitch,
            buf: buf.chars().collect(),
        }
    }
}

const STEPS: &[(i8, i8)] = &[
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn pos_seq(step: (i8, i8), len: u8) -> Vec<(i8, i8)> {
    let (drow, dcol) = step;
    (0..len).map(|n| (n as i8 * drow, n as i8 * dcol)).collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let ws = WordSearch::load(input);
    let mut cnt = 0u32;

    let needle = "XMAS";
    let needle_len = needle.len() as u8;

    for row in 0..ws.height() as isize {
        for col in 0..ws.width as isize {
            'check_pos: for step in STEPS.iter().copied() {
                let seq = pos_seq(step, needle_len);
                for (ch, (row_dif, col_dif)) in needle.chars().zip(seq.into_iter()) {
                    let ch_row = row + row_dif as isize;
                    if ch_row.is_negative() || ch_row >= ws.height() as isize {
                        continue 'check_pos;
                    }

                    let ch_col = col + col_dif as isize;
                    if ch_col.is_negative() || ch_col >= ws.width as isize {
                        continue 'check_pos;
                    }

                    if ws.buf[ws.width * ch_row as usize + ch_col as usize] != ch {
                        continue 'check_pos;
                    }
                }

                cnt += 1;
            }
        }
    }

    cnt
}

fn part2(input: &mut dyn Read) -> u32 {
    let ws = WordSearch::load(input);
    let mut cnt = 0u32;

    for row in 1..(ws.height() - 1) {
        for col in 1..(ws.width - 1) {
            if ws.buf[ws.width * row as usize + col as usize] != 'A' {
                continue;
            }

            let topleft = ws.buf[ws.width * (row - 1) + col - 1];
            let bottomleft = ws.buf[ws.width * (row + 1) + col - 1];
            let topright = ws.buf[ws.width * (row - 1) + col + 1];
            let bottomright = ws.buf[ws.width * (row + 1) + col + 1];

            cnt += match (topleft, bottomright, bottomleft, topright) {
                ('M', 'S', 'M', 'S') => 1,
                ('M', 'S', 'S', 'M') => 1,
                ('S', 'M', 'M', 'S') => 1,
                ('S', 'M', 'S', 'M') => 1,
                _ => 0,
            };
        }
    }

    cnt
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
        let mut f = File::open("input/d04-e.txt").unwrap();
        let ws = WordSearch::load(&mut f);
        assert_eq!(ws.width, 10);
        assert_eq!(ws.height(), 10);
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d04-e.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 18);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d04-f.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 2434);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d04-e.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 9);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d04-f.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 1835);
    }
}
