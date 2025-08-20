use aoc24::{
    cli::Cli,
    day::*,
    harness::{Day, Part},
};

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // `cli.part` guaranteed to be 1 or 2 via `clap`
    let part = Part::try_from(cli.part).expect("`clap` to parse valid part");

    let solution = match cli.day {
        1 => D1::run(part),
        2 => D2::run(part),
        3 => D3::run(part),
        4 => D4::run(part),
        5 => D5::run(part),
        6 => D6::run(part),
        7 => D7::run(part),
        8 => D8::run(part),
        9 => D9::run(part),
        _ => return Err(anyhow!("Day {} not yet solved", cli.day)),
    }?;

    println!("Day {} part {} solution: {}", cli.day, cli.part, solution);
    Ok(())
}
