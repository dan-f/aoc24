use aoc24::{cli::Cli, day, part::Part, solution};

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // `cli.part` guaranteed to be 1 or 2 via `clap`
    let part = Part::try_from(cli.part).expect("`clap` to parse valid part");

    let solution = match cli.day {
        1 => solution::solve_day::<day::D1>(part),
        _ => return Err(anyhow!("Day {} not yet solved", cli.day)),
    }?;

    println!("Day {} part {} solution: {}", cli.day, cli.part, solution);
    Ok(())
}
