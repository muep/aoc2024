use std::io::{BufRead, BufReader, Read};
use std::iter::successors;

fn int_sz(n: u64) -> u32 {
    successors(Some(10u64), |f| Some(f * 10))
        .enumerate()
        .find(|(_, top)| n < *top)
        .unwrap()
        .0 as u32
        + 1
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Cat => a * 10u64.pow(int_sz(b)) + b,
        }
    }
}

struct Candidates {
    first_op: Op,
    last_op: Op,
    ops: Vec<Op>,
    buf: Vec<Op>,
}

impl Candidates {
    fn with_ops_and_count(ops: Vec<Op>, cnt: usize) -> Candidates {
        let first_op = *ops.first().unwrap();
        Candidates {
            first_op,
            last_op: *ops.last().unwrap(),
            ops,
            buf: vec![first_op; cnt],
        }
    }
    fn next_op(&self, op: Op) -> Op {
        let cur_index = self.ops.iter().position(|p| *p == op).unwrap();
        let next_index = (cur_index + 1) % self.ops.len();
        self.ops[next_index]
    }
}

impl Iterator for Candidates {
    type Item = Vec<Op>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            let out = self.buf.clone();

            for n in 0..self.buf.len() {
                self.buf[n] = self.next_op(self.buf[n]);

                if self.buf[n] != self.first_op {
                    break;
                }
            }

            if out.iter().all(|p| *p == self.last_op) {
                self.buf.clear();
            }

            Some(out)
        }
    }
}

fn equation_from_line(line: &str) -> (u64, Vec<u64>) {
    let mut res_and_params = line.split(": ");
    let result = res_and_params.next().unwrap().parse::<u64>().unwrap();
    let params = res_and_params
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    (result, params)
}

fn check_params_and_ops(params: &[u64], ops: &[Op], result: u64) -> bool {
    params[1..]
        .iter()
        .zip(ops)
        .scan((false, params[0]), |(terminate, acc), (param, op)| {
            *acc = op.apply(*acc, *param);

            let had_terminate = *terminate;

            if *acc > result {
                *terminate = true;
            }

            if had_terminate {
                None
            } else {
                Some(*acc)
            }
        })
        .last()
        == Some(result)
}

fn part(ops: Vec<Op>, input: &mut dyn Read) -> u64 {
    BufReader::new(input)
        .lines()
        .map(|l| equation_from_line(&l.unwrap()))
        .filter(|(result, params)| {
            Candidates::with_ops_and_count(ops.clone(), params.len() - 1)
                .into_iter()
                .any(|ops| check_params_and_ops(params, &ops, *result))
        })
        .map(|(r, _)| r)
        .sum()
}

fn part1(input: &mut dyn Read) -> u64 {
    part(vec![Op::Add, Op::Mul], input)
}

fn part2(input: &mut dyn Read) -> u64 {
    part(vec![Op::Add, Op::Mul, Op::Cat], input)
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
    use std::collections::HashSet;
    use std::fs::File;

    #[test]
    fn test_int_sz() {
        assert_eq!(int_sz(9), 1);
        assert_eq!(int_sz(10), 2);
        assert_eq!(int_sz(11), 2);
        assert_eq!(int_sz(99), 2);
        assert_eq!(int_sz(100), 3);
    }

    #[test]
    fn test_equation_from_line() {
        assert_eq!(
            equation_from_line("3267: 81 40 27"),
            (3267, vec![81, 40, 27])
        );
    }

    #[test]
    fn test_check_1() {
        assert!(check_params_and_ops(
            &[81, 40, 27],
            &[Op::Add, Op::Mul],
            3267
        ));
    }

    #[test]
    fn test_check_2() {
        assert_eq!(
            check_params_and_ops(
                &[89, 4, 78, 6, 1],
                &[Op::Mul, Op::Add, Op::Mul, Op::Add],
                2604
            ),
            false
        );
    }

    #[test]
    fn test_op_candidates_v1_iter_3() {
        let expected: HashSet<Vec<Op>> = HashSet::from_iter(
            vec![
                vec![Op::Add, Op::Add, Op::Add],
                vec![Op::Add, Op::Add, Op::Mul],
                vec![Op::Add, Op::Mul, Op::Add],
                vec![Op::Add, Op::Mul, Op::Mul],
                vec![Op::Mul, Op::Add, Op::Add],
                vec![Op::Mul, Op::Add, Op::Mul],
                vec![Op::Mul, Op::Mul, Op::Add],
                vec![Op::Mul, Op::Mul, Op::Mul],
            ]
            .into_iter(),
        );
        let actual: HashSet<Vec<Op>> =
            HashSet::from_iter(Candidates::with_ops_and_count(vec![Op::Add, Op::Mul], 3));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_op_candidatesv2_iter_2() {
        let expected: HashSet<Vec<Op>> = HashSet::from_iter(
            vec![
                vec![Op::Add, Op::Add],
                vec![Op::Add, Op::Mul],
                vec![Op::Add, Op::Cat],
                vec![Op::Mul, Op::Add],
                vec![Op::Mul, Op::Mul],
                vec![Op::Mul, Op::Cat],
                vec![Op::Cat, Op::Add],
                vec![Op::Cat, Op::Mul],
                vec![Op::Cat, Op::Cat],
            ]
            .into_iter(),
        );
        let actual: HashSet<Vec<Op>> = HashSet::from_iter(Candidates::with_ops_and_count(
            vec![Op::Add, Op::Mul, Op::Cat],
            2,
        ));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_size() {
        let mut f = File::open("input/d07-f.txt").unwrap();
        let max_len = BufReader::new(&mut f)
            .lines()
            .map(|l| equation_from_line(&l.unwrap()).1.len())
            .max()
            .unwrap();
        assert_eq!(max_len, 12);
    }

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d07-e.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d07-f.txt").unwrap();
        let result = part1(&mut f);
        assert_eq!(result, 850435817339);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d07-e.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d07-f.txt").unwrap();
        let result = part2(&mut f);
        assert_eq!(result, 104824810233437);
    }
}
