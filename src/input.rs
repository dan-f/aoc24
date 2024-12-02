use std::io::{self, BufRead};

use thiserror::Error;

/// Pre-processed solution input
pub trait SolutionInput: Sized {
    /// Parse from a `BufRead`
    fn read(reader: impl BufRead) -> Result<Self, InputError>;
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("input file not available or failed to read")]
    FileIo(#[from] io::Error),
    #[error("failed to parse input: {msg}")]
    InvalidInput {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
