use std::{io::BufRead, str::FromStr};

use anyhow::Context;

use crate::Day;

pub struct D1;

impl Default for D1 {
    fn default() -> Self {
        D1
    }
}

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

impl Input {
    fn read(reader: impl BufRead) -> anyhow::Result<Self> {
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

impl Day for D1 {
    type Input = Input;
    type Outcome = u32;

    fn input_fname(&self) -> &'static str {
        "1"
    }

    fn parse(&self, reader: impl BufRead) -> anyhow::Result<Self::Input> {
        Input::read(reader)
    }

    fn solve(&self, input: Self::Input) -> Self::Outcome {
        let mut outcome = 0;

        for (l, r) in input.l.iter().zip(input.r.iter()) {
            outcome += l.abs_diff(*r)
        }

        outcome
    }

    fn fmt(&self, soln: &Self::Outcome) -> String {
        soln.to_string()
    }
}
