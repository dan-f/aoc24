use crate::Day;

pub struct D1;

impl Day for D1 {
    type P1 = p1::P1;
    type P2 = p2::P2;

    fn num() -> u8 {
        1
    }
}

pub mod p1 {
    use std::str::FromStr;

    use anyhow::Context;

    use crate::{FromBufRead, Part, PartSolution};

    pub struct P1;

    pub struct Input {
        // observation! each num is 5 digits, meaning we can represent these lists
        // with constant-space. (100,000 entries)
        //
        // we could almost represent them with a 32-bit number *except* for the fact
        // that we could have multiple versions of the same number.
        //
        // So we could do something like a 100,000-long array where each element has
        // an increment counter, and we iterate through them by:
        // - when at an empty bucket, skip until we reach a full bucket
        // - "count" a bucket by decrementing it
        //
        // This also means we don't have to do n * log(n) sorting
        //
        l: Vec<u32>,
        r: Vec<u32>,
    }

    impl FromBufRead for Input {
        fn from_reader(reader: impl std::io::BufRead) -> anyhow::Result<Self> {
            let mut l: Vec<u32> = vec![];
            let mut r: Vec<u32> = vec![];

            for line in reader.lines() {
                let line = line.context("Failed to read line")?;
                let mut nums = line.split_whitespace();
                let l_num = nums
                    .next()
                    .and_then(|x| u32::from_str(x).ok())
                    .context("Line in input not well-formatted (expected two numbers)")?;
                let r_num = nums
                    .next()
                    .and_then(|x| u32::from_str(x).ok())
                    .context("Line in input not well-formatted (expected two numbers)")?;
                l.push(l_num);
                r.push(r_num);
            }

            l.sort();
            r.sort();

            Ok(Self { l, r })
        }
    }

    impl PartSolution for P1 {
        type Input = Input;
        type Output = u32;

        fn part() -> Part {
            Part::One
        }

        fn solve(input: Self::Input) -> Self::Output {
            let mut outcome = 0;

            for (l, r) in input.l.iter().zip(input.r.iter()) {
                outcome += l.abs_diff(*r)
            }

            outcome
        }
    }
}

pub mod p2 {
    use std::str::FromStr;

    use anyhow::Context;

    use crate::{FromBufRead, Part, PartSolution};

    pub struct Input {
        l: [bool; 100_000],
        r: Vec<u32>,
    }

    impl Input {
        pub fn l_contains(&self, n: u32) -> bool {
            self.l[n as usize]
        }
    }

    impl FromBufRead for Input {
        fn from_reader(reader: impl std::io::BufRead) -> anyhow::Result<Self> {
            let mut l = [false; 100_000];
            let mut r: Vec<u32> = vec![];

            for line in reader.lines() {
                let line = line.context("Failed to read line")?;
                let mut nums = line.split_whitespace();
                let l_num = nums
                    .next()
                    .and_then(|x| u32::from_str(x).ok())
                    .context("Line in input not well-formatted (expected two numbers)")?;
                let r_num = nums
                    .next()
                    .and_then(|x| u32::from_str(x).ok())
                    .context("Line in input not well-formatted (expected two numbers)")?;
                l[l_num as usize] = true;
                r.push(r_num);
            }

            Ok(Self { l, r })
        }
    }

    pub struct P2;

    impl PartSolution for P2 {
        type Input = Input;
        type Output = u32;

        fn part() -> Part {
            Part::Two
        }

        fn solve(input: Self::Input) -> Self::Output {
            let mut out = 0;

            for n in input.r.iter() {
                if input.l[*n as usize] {
                    out += n
                }
            }

            out
        }
    }
}
