use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
};

pub mod d1;

// TODO probably something like:
// - refactor this whole type into something called "Solution" and rename `Solution` associated type to `Answer`
// - then have `Day` have two solutions; one for pt1, one for pt2
//
// TODO refactor out anyhow::Error as this is "library" code
pub trait Day: Default {
    type Input;
    type Outcome;

    /// The filename of the day's input relative to the local `input/` dir
    fn input_fname(&self) -> &'static str;

    fn parse(&self, reader: impl BufRead) -> anyhow::Result<Self::Input>;

    /// Compute the solution given a buffered reader over the input file
    fn solve(&self, input: Self::Input) -> Self::Outcome;

    /// Format a solution for submission
    fn fmt(&self, soln: &Self::Outcome) -> String;
}

pub fn solve_day(day: impl Day) -> anyhow::Result<String> {
    let mut input_path = current_dir()?;
    input_path.push("input");
    input_path.push(day.input_fname());

    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let input = day.parse(&mut reader)?;
    let soln = day.solve(input);
    Ok(day.fmt(&soln))
}
