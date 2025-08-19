use std::str::FromStr;

use crate::{
    harness::{iter, Day, InputError, Inputs, SolutionInput},
    parse,
};

pub struct D7;

impl Day for D7 {
    type P1<'a> = p1::P1;

    type P2<'a> = p2::P2;

    fn day() -> u8 {
        7
    }
}

pub mod p1 {
    use crate::{
        day::d7::Equation,
        harness::{Inputs, Solution},
    };

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Box<dyn Inputs<Equation> + 'a>;

        type Output = u64;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            input.fold_solve(0, |sum, eq| {
                Ok(if eq.can_solve(false) {
                    sum + eq.target
                } else {
                    sum
                })
            })
        }
    }
}

pub mod p2 {
    use crate::{
        day::d7::Equation,
        harness::{Inputs, Solution},
    };

    pub struct P2;

    impl<'a> Solution<'a> for P2 {
        type Input = Box<dyn Inputs<Equation> + 'a>;

        type Output = u64;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            input.fold_solve(0, |sum, eq| {
                Ok(if eq.can_solve(true) {
                    sum + eq.target
                } else {
                    sum
                })
            })
        }
    }
}

pub struct Equation {
    target: u64,
    terms: Vec<u64>,
}

impl Equation {
    pub fn can_solve(&self, use_concat: bool) -> bool {
        let mut stack: Vec<(u64, &[u64])> = vec![(0, &self.terms)];

        while let Some((acc, terms)) = stack.pop() {
            // backtrack when:
            // - we've exceeded the target already (no ability to reduce)
            // - we've hit the end of our search path (used all operators) and did not reach the target
            if acc > self.target || (acc != self.target && terms.is_empty()) {
                continue;
            }
            if acc == self.target && terms.is_empty() {
                return true;
            }
            let next_term = terms.first().unwrap();
            stack.push((acc + next_term, &terms[1..]));
            stack.push((acc * next_term, &terms[1..]));
            if use_concat {
                stack.push((concat(acc, *next_term), &terms[1..]));
            }
        }

        false
    }
}

impl FromStr for Equation {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let target = split
            .next()
            .ok_or(InputError::InvalidInput {
                msg: format!("Bad equation: '{}'", s),
                source: None,
            })
            .and_then(parse::parse_u64)?;
        let terms: Vec<_> = split
            .next()
            .ok_or(InputError::InvalidInput {
                msg: format!("Bad equation: '{}'", s),
                source: None,
            })?
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(parse::parse_u64)
            .collect::<Result<_, _>>()?;
        Ok(Equation { target, terms })
    }
}

impl<'a> SolutionInput<'a> for Box<dyn Inputs<Equation> + 'a> {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        Ok(Box::new(
            iter::lines(reader).map(|line| Equation::from_str(&line?)),
        ))
    }
}

pub fn concat(x: u64, y: u64) -> u64 {
    let y_digits = y.ilog10() + 1;
    x * 10_u64.pow(y_digits) + y
}
