use std::io::BufRead;

pub trait Day {
    type Solution;

    /// The filename of the day's input
    fn input_fname() -> &'static str;

    /// Compute the solution given a buffered reader over the input file
    fn solve(reader: impl BufRead) -> Self::Solution;

    /// Format a solution for submission
    fn fmt(soln: &Self::Solution) -> String;
}
