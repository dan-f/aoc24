use crate::{
    harness::{input, Day, InputError},
    parse,
};

pub struct D1;

impl Day for D1 {
    type P1<'a> = p1::P1;
    type P2<'a> = p2::P2;

    fn day() -> u8 {
        1
    }
}

pub mod p1 {
    use crate::harness::{input, Result, Solution, SolutionInput};

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Input;
        type Output = u32;

        fn solve(input: Self::Input) -> Result<Self::Output> {
            let ls = input.l.iter();
            let rs = input.r.iter();
            Ok(ls.zip(rs).map(|(l, r)| l.abs_diff(*r)).sum())
        }
    }

    pub struct Input {
        l: Vec<u32>,
        r: Vec<u32>,
    }

    impl<'a> SolutionInput<'a> for Input {
        fn read(reader: impl std::io::BufRead + 'a) -> input::Result<Self> {
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
}

pub mod p2 {
    use crate::harness::{input, iter, Result, Solution, SolutionInput};

    use super::parse_line;

    pub struct P2;

    impl<'a> Solution<'a> for P2 {
        type Input = Input;
        type Output = u32;

        fn solve(input: Self::Input) -> Result<Self::Output> {
            Ok(input.r.iter().filter(|n| input.l[**n as usize]).sum())
        }
    }

    pub struct Input {
        l: [bool; 100_000],
        r: Vec<u32>,
    }

    impl<'a> SolutionInput<'a> for Input {
        fn read(reader: impl std::io::BufRead + 'a) -> input::Result<Self> {
            let mut l = [false; 100_000];
            let mut r: Vec<u32> = vec![];

            for (idx, line) in iter::lines(reader).enumerate() {
                let (l_num, r_num) = parse_line(idx, line?)?;
                l[l_num as usize] = true;
                r.push(r_num);
            }

            Ok(Self { l, r })
        }
    }
}

fn parse_line(line_num: usize, line: String) -> input::Result<(u32, u32)> {
    let mut nums = line.split_whitespace();
    let l_num = parse_num(line_num, nums.next())?;
    let r_num = parse_num(line_num, nums.next())?;
    Ok((l_num, r_num))
}

fn parse_num(line_num: usize, num: Option<&str>) -> input::Result<u32> {
    let num = num.ok_or(InputError::InvalidInput {
        msg: format!("Missing num on line {}", line_num),
        source: None,
    })?;

    parse::parse_u32(num)
}
