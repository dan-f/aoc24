use std::{fmt::Display, io::BufRead};

use thiserror::Error;

use super::input::{InputError, SolutionInput};

/// Solution for a given daily exercise's part component
pub trait Solution<'a> {
    /// Pre-processed input
    type Input: SolutionInput<'a>;
    /// Exercise result
    type Output: Display;

    /// Compute the solution from the pre-processed input
    fn solve(input: Self::Input) -> Result<Self::Output>;

    /// Run the solution over an input buffer
    fn run(reader: impl BufRead + 'a) -> Result<String> {
        let input = Self::Input::read(reader)?;
        let output = Self::solve(input)?;
        Ok(format!("{}", output))
    }
}

/// Top-level result returned by a solution
pub type Result<T> = std::result::Result<T, Error>;

/// Top-level errors encountered when computing a solution
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    FileInput(#[from] InputError),

    #[error(transparent)]
    SolutionError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
