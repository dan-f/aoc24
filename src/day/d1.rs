use crate::{
    harness::{Day, InputError},
    parse,
};

pub struct D1;

impl Day for D1 {
    type P1 = p1::P1;
    type P2 = p2::P2;

    fn num() -> u8 {
        1
    }
}

pub mod p1 {
    use crate::harness::{InputError, Solution, SolutionInput, SolveResult};

    pub struct P1;

    pub struct Input {
        l: Vec<u32>,
        r: Vec<u32>,
    }

    impl SolutionInput for Input {
        fn read(reader: impl std::io::BufRead) -> Result<Self, InputError> {
            let mut l: Vec<u32> = vec![];
            let mut r: Vec<u32> = vec![];

            for (idx, line) in reader.lines().enumerate() {
                let (l_num, r_num) = super::parse_line(idx, line?)?;
                l.push(l_num);
                r.push(r_num);
            }

            l.sort();
            r.sort();

            Ok(Self { l, r })
        }
    }

    impl Solution for P1 {
        type Input = Input;
        type Output = u32;

        fn solve(input: Self::Input) -> SolveResult<Self::Output> {
            let ls = input.l.iter();
            let rs = input.r.iter();
            Ok(ls.zip(rs).map(|(l, r)| l.abs_diff(*r)).sum())
        }
    }
}

pub mod p2 {
    use crate::harness::{InputError, Solution, SolutionInput, SolveResult};

    use super::parse_line;

    pub struct Input {
        l: [bool; 100_000],
        r: Vec<u32>,
    }

    impl SolutionInput for Input {
        fn read(reader: impl std::io::BufRead) -> Result<Self, InputError> {
            let mut l = [false; 100_000];
            let mut r: Vec<u32> = vec![];

            for (idx, line) in reader.lines().enumerate() {
                let (l_num, r_num) = parse_line(idx, line?)?;
                l[l_num as usize] = true;
                r.push(r_num);
            }

            Ok(Self { l, r })
        }
    }

    pub struct P2;

    impl Solution for P2 {
        type Input = Input;
        type Output = u32;

        fn solve(input: Self::Input) -> SolveResult<Self::Output> {
            Ok(input.r.iter().filter(|n| input.l[**n as usize]).sum())
        }
    }
}

fn parse_line(line_num: usize, line: String) -> Result<(u32, u32), InputError> {
    let mut nums = line.split_whitespace();
    let l_num = parse_num(line_num, nums.next())?;
    let r_num = parse_num(line_num, nums.next())?;
    Ok((l_num, r_num))
}

fn parse_num(line_num: usize, num: Option<&str>) -> Result<u32, InputError> {
    let num = num.ok_or(InputError::InvalidInput {
        msg: format!("Missing num on line {}", line_num),
        source: None,
    })?;

    parse::parse_u32(num)
}
