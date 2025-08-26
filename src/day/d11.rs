use std::collections::VecDeque;

use crate::{
    harness::{Day, Solution, SolutionInput},
    parse,
};

pub struct D11;

impl Day for D11 {
    type P1<'a> = P1;

    type P2<'a> = P1;

    fn day() -> u8 {
        11
    }
}

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Vec<u64>;

    type Output = u64;

    fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut count = 0;

        let mut queue: VecDeque<(u64, u64)> = input.iter().map(|n| (*n, 0)).collect();
        while let Some((n, applications)) = queue.pop_front() {
            if applications == 25 {
                count += 1;
                continue;
            }

            if n == 0 {
                queue.push_back((1, applications + 1));
            } else if let Some((left, right)) = split_digits(n) {
                queue.push_back((left, applications + 1));
                queue.push_back((right, applications + 1));
            } else {
                queue.push_back((n * 2024, applications + 1));
            }
        }

        Ok(count)
    }
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    if n == 0 {
        return None;
    }

    let digits = n.ilog10() + 1;
    if digits % 2 != 0 {
        return None;
    }

    let left = n / 10_u64.pow(digits / 2);
    let right = n % 10_u64.pow(digits / 2);
    Some((left, right))
}

impl<'a> SolutionInput<'a> for Vec<u64> {
    fn read(mut reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let mut v = vec![];
        for token in buf.trim().split_ascii_whitespace() {
            v.push(parse::parse_u64(token)?);
        }

        Ok(v)
    }
}
