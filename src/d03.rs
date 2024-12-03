use std::io::{BufReader, Read};

#[derive(Clone, Copy)]
enum State {
    Init,
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
}

impl State {
    fn step(&self, b: u8) -> State {
        use State::*;
        let c = b as char;
        match (*self, c, c.to_digit(10)) {
            (Init, 'm', _) => M,
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
            _ => Init,
        }
    }
}

fn part1(input: &mut dyn Read) -> u32 {
    BufReader::new(input)
        .bytes()
        .fold((State::Init, 0u32), |(state, acc), n| {
            match state.step(n.unwrap()) {
                State::ClosingParen(a, b) => (State::Init, acc + a * b),
                state => (state, acc),
            }
        })
        .1
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_part1_example() {
        let mut f = File::open("input/d03-e.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 161);
    }

    #[test]
    fn test_part1_full() {
        let mut f = File::open("input/d03-f.txt").unwrap();
        let safe_reports = part1(&mut f);
        assert_eq!(safe_reports, 166357705);
    }
}
