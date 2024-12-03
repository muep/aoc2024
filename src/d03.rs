use std::io::{BufReader, Read};

#[derive(Clone, Copy)]
enum State {
    Init(bool),
    M,
    Mu,
    Mul,
    OpeningParen,
    A1(u32),
    A2(u32),
    A3(u32),
    Comma(u32),
    B1(u32, u32),
    B2(u32, u32),
    B3(u32, u32),
    ClosingParen(u32, u32),
    D(bool),
    Do(bool),
    DoOpeningParen(bool),
    Don(bool),
    Donn(bool),
    Donnt(bool),
    DonntOpeningParen(bool),
}

impl State {
    fn step(&self, b: u8) -> State {
        use State::*;
        let c = b as char;
        match (*self, c, c.to_digit(10)) {
            (Init(true), 'm', _) => M,
            (ClosingParen(_, _), 'm', _) => M,
            (M, 'u', _) => Mu,
            (Mu, 'l', _) => Mul,
            (Mul, '(', _) => OpeningParen,
            (OpeningParen, _, Some(d)) => A1(d),
            (A1(a), _, Some(d)) => A2(10 * a + d),
            (A1(a), ',', _) => Comma(a),
            (A2(a), _, Some(d)) => A3(10 * a + d),
            (A2(a), ',', _) => Comma(a),
            (A3(a), ',', _) => Comma(a),
            (Comma(a), _, Some(d)) => B1(a, d),
            (B1(a, b), _, Some(d)) => B2(a, 10 * b + d),
            (B1(a, b), ')', _) => ClosingParen(a, b),
            (B2(a, b), _, Some(d)) => B3(a, 10 * b + d),
            (B2(a, b), ')', _) => ClosingParen(a, b),
            (B3(a, b), ')', _) => ClosingParen(a, b),
            _ => Init(true),
        }
    }

    fn step2(&self, b: u8) -> State {
        use State::*;
        let c = b as char;
        match (*self, c, c.to_digit(10)) {
            (Init(true), 'm', _) => M,
            (Init(enabled), 'd', _) => D(enabled),
            (Init(enabled), _, _) => Init(enabled),
            (ClosingParen(_, _), 'm', _) => M,
            (ClosingParen(_, _), 'd', _) => D(true),
            (M, 'u', _) => Mu,
            (M, 'd', _) => D(true),
            (Mu, 'l', _) => Mul,
            (Mu, 'd', _) => D(true),
            (Mul, '(', _) => OpeningParen,
            (Mul, 'd', _) => D(true),
            (OpeningParen, _, Some(d)) => A1(d),
            (OpeningParen, 'd', _) => D(true),
            (A1(a), _, Some(d)) => A2(10 * a + d),
            (A1(a), ',', _) => Comma(a),
            (A1(_), 'd', _) => D(true),
            (A2(a), _, Some(d)) => A3(10 * a + d),
            (A2(a), ',', _) => Comma(a),
            (A2(_), 'd', _) => D(true),
            (A3(a), ',', _) => Comma(a),
            (A3(_), 'd', _) => D(true),
            (Comma(a), _, Some(d)) => B1(a, d),
            (Comma(_), 'd', _) => D(true),
            (B1(a, b), _, Some(d)) => B2(a, 10 * b + d),
            (B1(a, b), ')', _) => ClosingParen(a, b),
            (B1(_, _), 'd', _) => D(true),
            (B2(a, b), _, Some(d)) => B3(a, 10 * b + d),
            (B2(a, b), ')', _) => ClosingParen(a, b),
            (B2(_, _), 'd', _) => D(true),
            (B3(a, b), ')', _) => ClosingParen(a, b),
            (B3(_, _), 'd', _) => D(true),
            (D(true), 'm', _) => M,
            (D(enabled), 'o', _) => Do(enabled),
            (D(enabled), _, _) => Init(enabled),
            (Do(true), 'm', _) => M,
            (Do(enabled), '(', _) => DoOpeningParen(enabled),
            (Do(enabled), 'n', _) => Don(enabled),
            (Do(enabled), _, _) => Init(enabled),
            (DoOpeningParen(true), 'm', _) => M,
            (DoOpeningParen(_), ')', _) => Init(true),
            (DoOpeningParen(enabled), _, _) => Init(enabled),
            (Don(true), 'm', _) => M,
            (Don(enabled), '\'', _) => Donn(enabled),
            (Don(enabled), _, _) => Init(enabled),
            (Donn(true), 'm', _) => M,
            (Donn(enabled), 't', _) => Donnt(enabled),
            (Donn(enabled), _, _) => Init(enabled),
            (Donnt(true), 'm', _) => M,
            (Donnt(enabled), '(', _) => DonntOpeningParen(enabled),
            (Donnt(enabled), _, _) => Init(enabled),
            (DonntOpeningParen(true), 'm', _) => M,
            (DonntOpeningParen(_), ')', _) => Init(false),
            (DonntOpeningParen(enabled), _, _) => Init(enabled),
            _ => Init(true), // Boldly assuming that all the disabled states were handled separately
        }
    }
}

fn part1(input: &mut dyn Read) -> u32 {
    BufReader::new(input)
        .bytes()
        .fold((State::Init(true), 0u32), |(state, acc), n| {
            match state.step(n.unwrap()) {
                State::ClosingParen(a, b) => (State::Init(true), acc + a * b),
                state => (state, acc),
            }
        })
        .1
}

fn part2(input: &mut dyn Read) -> u32 {
    BufReader::new(input)
        .bytes()
        .fold((State::Init(true), 0u32), |(state, acc), n| {
            match state.step2(n.unwrap()) {
                State::ClosingParen(a, b) => (State::Init(true), acc + a * b),
                state => (state, acc),
            }
        })
        .1
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
        let mut f = File::open("input/d03-e1.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 161);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d03-f.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 166357705);
    }

    #[test]
    fn test_part2_example() {
        let mut f = File::open("input/d03-e2.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 48);
    }

    #[test]
    fn test_part2_full() {
        let mut f = File::open("input/d03-f.txt").unwrap();
        let safe_reports = part2(&mut f);
        assert_eq!(safe_reports, 88811886);
    }
}
