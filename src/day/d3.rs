use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    harness::{Day, Part},
    parse::parse_u32,
};

pub struct D3;

impl Day for D3 {
    type P1<'a> = p1::P1;
    type P2<'a> = p1::P1;

    fn day() -> u8 {
        3
    }
}

pub mod p1 {
    use crate::{
        day::d3::Machine,
        harness::{input, iter, Inputs, Part, Solution, SolutionInput},
    };

    use super::{find_instructions, Instruction};

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Box<dyn Inputs<Instruction> + 'a>;
        type Output = u32;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            input
                .fold_solve(Machine::new(Part::One), |exec, instruction| {
                    Ok(exec.execute(instruction))
                })
                .map(|exec| exec.count)
        }
    }

    impl<'a> SolutionInput<'a> for Box<dyn Inputs<Instruction> + 'a> {
        fn read(reader: impl std::io::BufRead + 'a) -> input::Result<Self> {
            Ok(Box::new(iter::lines(reader).flat_map(|line| {
                if line.is_ok() {
                    find_instructions(&line.unwrap())
                        .into_iter()
                        .map(Result::Ok)
                        .collect()
                } else {
                    vec![Err(line.err().unwrap())]
                }
            })))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Clone, Copy)]
pub struct Machine {
    part: Part,
    resumed: bool,
    count: u32,
}

impl Machine {
    pub fn new(part: Part) -> Self {
        Self {
            part,
            resumed: false,
            count: 0,
        }
    }

    pub fn execute(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Mul(x, y) => {
                if self.allow_mul() {
                    let mut new = self.clone();
                    new.count += x * y;
                    new
                } else {
                    self
                }
            }
            Instruction::Do => {
                let mut new = self.clone();
                new.resumed = true;
                new
            }
            Instruction::Dont => {
                let mut new = self.clone();
                new.resumed = false;
                new
            }
        }
    }

    pub fn allow_mul(&self) -> bool {
        self.part == Part::One || self.resumed
    }
}

fn find_instructions(haystack: &str) -> Vec<Instruction> {
    // static MUL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    static MUL_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?<mul>mul\((?<mul_x>\d{1,3}),(?<mul_y>\d{1,3})\))|(?<do>do\(\))|(?<dont>don\'t\(\))",
        )
        .unwrap()
    });
    MUL_RE
        .captures_iter(haystack)
        .map(|caps| {
            if caps.name("mul").is_some() {
                Instruction::Mul(
                    parse_u32(&caps["mul_x"]).unwrap(),
                    parse_u32(&caps["mul_y"]).unwrap(),
                )
            } else if caps.name("do").is_some() {
                Instruction::Do
            } else {
                Instruction::Dont
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day::d3::{find_instructions, Instruction};

    #[test]
    fn test_parse_muls() {
        let haystack = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ],
            find_instructions(haystack)
        );
    }
}
