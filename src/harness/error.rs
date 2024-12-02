use std::result;

use thiserror::Error;

use super::input::InputError;

pub type Result<T> = result::Result<T, Error>;

/// Top-level errors encountered when computing a solution
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    FileInput(#[from] InputError),

    #[error(transparent)]
    SolutionError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
