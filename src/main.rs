use std::{env, str::FromStr};

use anyhow::{anyhow, Context, Result};
use aoc24::{d1::D1, solve_day};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let day_arg = args.get(1).context("Missing day argument")?;
    let day_num: u8 =
        FromStr::from_str(day_arg).context(format!("Failed to parse day from {}", day_arg))?;

    let day = match day_num {
        1 => D1::default(),
        _ => return Err(anyhow!("No solution for day {}", day_num)),
    };

    println!("Day {} solution: {}", day_num, solve_day(day)?);

    Ok(())
}
