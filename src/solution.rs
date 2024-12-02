use std::{
    env::current_dir,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    error,
    input::{InputError, SolutionInput},
    part::Part,
};

/// Daily two-part exercise
pub trait DaySolution {
    type P1: PartSolution;
    type P2: PartSolution;

    /// The day (1-indexed)
    fn num() -> u8;
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
    fn solve(input: Self::Input) -> SolveResult<Self::Output>;
}

/// Compute the formatted result for the given `part`
pub fn solve_day<D: DaySolution>(part: Part) -> error::Result<String> {
    let mut input_path = current_dir().map_err(|err| InputError::from(err))?;
    input_path.push("input");
    input_path.push(format!("d{}p{}", D::num(), part.num()));

    let input_file = File::open(input_path).map_err(|err| InputError::from(err))?;
    let reader = BufReader::new(input_file);

    match part {
        Part::One => solve_part::<D::P1>(reader),
        Part::Two => solve_part::<D::P2>(reader),
    }
}

pub type SolveResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn solve_part<P: PartSolution>(reader: impl BufRead) -> error::Result<String> {
    let input = P::Input::read(reader)?;
    let output = P::solve(input)?;
    Ok(format!("{}", output))
}
