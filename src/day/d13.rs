use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    harness::{iter, Day, InputError, Inputs, Solution, SolutionInput},
    parse,
};

pub struct D13;

impl Day for D13 {
    type P1<'a> = P1;

    type P2<'a> = P1;

    fn day() -> u8 {
        13
    }
}

static BTN_PRESS_LIMIT: u64 = 100;

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Box<dyn Inputs<Machine> + 'a>;

    type Output = u64;

    fn solve(machines: Self::Input) -> crate::harness::Result<Self::Output> {
        machines.fold_solve(0, |tokens, machine| Ok(tokens + min_tokens(&machine)))
    }
}

fn min_tokens(machine: &Machine) -> u64 {
    for a_presses in 0..(BTN_PRESS_LIMIT + 1) {
        for b_presses in 0..(BTN_PRESS_LIMIT + 1) {
            let moved_by_a = (a_presses * machine.move_a.0, a_presses * machine.move_a.1);
            let moved_by_b = (b_presses * machine.move_b.0, b_presses * machine.move_b.1);
            let end_position = (moved_by_a.0 + moved_by_b.0, moved_by_a.1 + moved_by_b.1);
            if end_position == machine.prize_position {
                return a_presses * 3 + b_presses;
            }
        }
    }

    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SearchState {
    position: (u64, u64),
    tokens_spent: u64,
    button_presses: u64,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            position: (0, 0),
            tokens_spent: 0,
            button_presses: 0,
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    move_a: (u64, u64),
    move_b: (u64, u64),
    prize_position: (u64, u64),
}

impl TryFrom<&str> for Machine {
    type Error = InputError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        static BTN_A_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Button A: X\+(?<X>\d+), Y\+(?<Y>\d+)").unwrap());
        static BTN_B_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Button B: X\+(?<X>\d+), Y\+(?<Y>\d+)").unwrap());
        static PRIZE_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Prize: X=(?<X>\d+), Y=(?<Y>\d+)").unwrap());

        let mk_err = || InputError::InvalidInput {
            msg: format!("Machine definition missing lines: {}", s),
            source: None,
        };

        let btn_a_caps = BTN_A_RE.captures(&s).ok_or_else(mk_err)?;
        let btn_b_caps = BTN_B_RE.captures(&s).ok_or_else(mk_err)?;
        let prize_caps = PRIZE_RE.captures(&s).ok_or_else(mk_err)?;

        Ok(Machine {
            move_a: (
                parse::parse_u64(&btn_a_caps["X"])?,
                parse::parse_u64(&btn_a_caps["Y"])?,
            ),
            move_b: (
                parse::parse_u64(&btn_b_caps["X"])?,
                parse::parse_u64(&btn_b_caps["Y"])?,
            ),
            prize_position: (
                parse::parse_u64(&prize_caps["X"])?,
                parse::parse_u64(&prize_caps["Y"])?,
            ),
        })
    }
}

impl<'a> SolutionInput<'a> for Box<dyn Inputs<Machine> + 'a> {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let mut lines = iter::lines(reader)
            .filter(|l_res| l_res.is_err() || l_res.as_ref().is_ok_and(|l| !l.trim().is_empty()));

        Ok(Box::new(std::iter::from_fn(move || {
            let strs: Vec<Result<_, _>> = lines.by_ref().take(3).collect();
            if strs.is_empty() {
                None
            } else {
                Some(
                    strs.into_iter()
                        .collect::<Result<String, _>>()
                        .and_then(|machine_def| Machine::try_from(machine_def.as_str())),
                )
            }
        })))
    }
}
