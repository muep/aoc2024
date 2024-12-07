use std::io::{BufRead, BufReader, Read};

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
            Op::Cat => format!("{a}{b}").parse::<u64>().unwrap(),
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

fn op_candidates_v1(sz: usize) -> Vec<Vec<Op>> {
    (0..(1u64 << sz))
        .map(|n| {
            (0..sz)
                .map(|pos| match (1 << pos) & n {
                    0 => Op::Add,
                    _ => Op::Mul,
                })
                .collect()
        })
        .collect()
}

fn op_candidates_v2_step(prev: Vec<Vec<Op>>) -> Vec<Vec<Op>> {
    let ops = [Op::Add, Op::Mul, Op::Cat];

    if prev.is_empty() {
        return ops.into_iter().map(|op| vec![op]).collect();
    }

    ops.into_iter()
        .map(|op| {
            prev.iter()
                .map(|op_sequence| {
                    let mut v = op_sequence.clone();
                    v.push(op);
                    v
                })
                .collect::<Vec<Vec<Op>>>()
        })
        .fold(Vec::new(), |mut a, mut b| {
            a.append(&mut b);
            a
        })
}

fn op_candidates_v2(sz: usize) -> Vec<Vec<Op>> {
    (0..sz).fold(Vec::new(), |p, _| op_candidates_v2_step(p))
}

fn apply_params_and_ops(params: &[u64], ops: &[Op]) -> u64 {
    params[1..]
        .iter()
        .zip(ops)
        .fold(params[0], |acc, (param, op)| op.apply(acc, *param))
}

fn part(op_candidates: fn(usize) -> Vec<Vec<Op>>, input: &mut dyn Read) -> u64 {
    BufReader::new(input)
        .lines()
        .map(|l| equation_from_line(&l.unwrap()))
        .filter(|(result, params)| {
            op_candidates(params.len() - 1)
                .into_iter()
                .any(|ops| apply_params_and_ops(params, &ops) == *result)
        })
        .map(|(r, _)| r)
        .sum()
}

fn part1(input: &mut dyn Read) -> u64 {
    part(op_candidates_v1, input)
}

fn part2(input: &mut dyn Read) -> u64 {
    part(op_candidates_v2, input)
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
    fn test_equation_from_line() {
        assert_eq!(
            equation_from_line("3267: 81 40 27"),
            (3267, vec![81, 40, 27])
        );
    }

    #[test]
    fn test_apply() {
        assert_eq!(
            apply_params_and_ops(&[81, 40, 27], &[Op::Add, Op::Mul]),
            3267
        );
    }

    #[test]
    fn test_op_candidates_v1_1() {
        let expected: HashSet<Vec<Op>> =
            HashSet::from_iter(vec![vec![Op::Add], vec![Op::Mul]].into_iter());
        let actual: HashSet<Vec<Op>> = HashSet::from_iter(op_candidates_v1(1).into_iter());

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_op_candidates_v1_3() {
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
        let actual: HashSet<Vec<Op>> = HashSet::from_iter(op_candidates_v1(3).into_iter());

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_op_candidatesv2_2() {
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
        let actual: HashSet<Vec<Op>> = HashSet::from_iter(op_candidates_v2(2).into_iter());

        assert_eq!(expected, actual)
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
