use std::{
    env::current_dir,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

/// Daily exercise component
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn num(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

impl TryFrom<u8> for Part {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> anyhow::Result<Self> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(anyhow!("Part must be 1 or 2, but got {}", value)),
        }
    }
}

/// Daily two-part exercise
pub trait DaySolution {
    type P1: PartSolution;
    type P2: PartSolution;

    /// The day (1-indexed)
    fn num() -> u8;

    /// Compute the formatted result for the given `part`
    fn solve(part: Part) -> anyhow::Result<String> {
        let mut input_path = current_dir()?;
        input_path.push("input");
        input_path.push(format!("d{}p{}", Self::num(), part.num()));

        let input_file = File::open(input_path)?;
        let reader = BufReader::new(input_file);

        match part {
            Part::One => Self::P1::solve_from_reader(reader),
            Part::Two => Self::P2::solve_from_reader(reader),
        }
    }
}

/// Solution for a given daily exercise's part component
pub trait PartSolution {
    /// Pre-processed input
    type Input: SolutionInput;
    /// Exercise result
    type Output: Display;

    /// The exercise part
    fn part() -> Part;

    /// Compute the solution from the pre-processed input
    fn solve(input: Self::Input) -> Self::Output;

    fn solve_from_reader(reader: impl BufRead) -> anyhow::Result<String> {
        let input = Self::Input::from_reader(reader)?;
        let output = Self::solve(input);
        Ok(format!("{}", output))
    }
}

pub trait SolutionInput: Sized {
    /// Parse from a `BufRead`
    fn from_reader(reader: impl BufRead) -> anyhow::Result<Self>;
}
