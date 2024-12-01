use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub mod d1;

pub trait Day: Default {
    type Solution;

    /// The filename of the day's input relative to the local `input/` dir
    fn input_fname(&self) -> &'static str;

    /// Compute the solution given a buffered reader over the input file
    fn solve(&mut self, reader: &mut impl BufRead) -> Self::Solution;

    /// Format a solution for submission
    fn fmt(&self, soln: &Self::Solution) -> String;
}

pub fn solve_day(mut day: impl Day) -> io::Result<String> {
    let mut input_path = current_dir()?;
    input_path.push("input");
    input_path.push(day.input_fname());

    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let soln = day.solve(&mut reader);
    Ok(day.fmt(&soln))
}
