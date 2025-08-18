//! Utilities for lazy iterator-based solutions.
//!
//! Solutions which don't require random access to the entire input can instead
//! iterate over inputs in chunks, e.g. line by line ([`lines`]).

use std::io::BufRead;

use super::{input, solution};

pub trait Inputs<Item>: Iterator<Item = input::Result<Item>> {
    fn fold_solve<T, F>(self, init: T, mut f: F) -> solution::Result<T>
    where
        Self: Sized,
        F: FnMut(T, Item) -> solution::Result<T>,
    {
        std::iter::Iterator::fold(self, Ok(init), |acc, cur| f(acc?, cur?))
    }
}

impl<T, Item> Inputs<Item> for T where T: Iterator<Item = input::Result<Item>> {}

/// Lines iterator of [`input::Result`] rather than [`std::io::Result`]
pub fn lines<R: BufRead>(reader: R) -> impl Inputs<String> {
    reader
        .lines()
        .map(|line| line.map_err(input::InputError::from))
}
