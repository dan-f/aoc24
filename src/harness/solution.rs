use std::{fmt::Display, io::BufRead};

use super::{error, input::SolutionInput};

pub type SolveResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Solution for a given daily exercise's part component
pub trait Solution {
    /// Pre-processed input
    type Input: SolutionInput;
    /// Exercise result
    type Output: Display;

    /// Compute the solution from the pre-processed input
    fn solve(input: Self::Input) -> SolveResult<Self::Output>;

    /// Run the solution over an input buffer
    fn run(reader: impl BufRead) -> error::Result<String> {
        let input = Self::Input::read(reader)?;
        let output = Self::solve(input)?;
        Ok(format!("{}", output))
    }
}
