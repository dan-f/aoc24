use std::{env, str::FromStr};

use anyhow::{anyhow, Context, Result};
use aoc24::{
    day,
    solution::{DaySolution, Part},
};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let day_num = parse_num_arg("day", args.get(1))?;
    let part_num = parse_num_arg("part", args.get(2))?;
    let part: Part = part_num.try_into()?;

    let solution = match day_num {
        0 => Err(anyhow!("Days begin at 1")),

        1 => day::D1::solve(part),

        26..=u8::MAX => Err(anyhow!("Day {} out of range", day_num)),
        _ => Err(anyhow!("Day {} not yet solved", day_num)),
    }?;

    println!("Day {} part {} solution: {}", day_num, part_num, solution);

    Ok(())
}

fn parse_num_arg(name: &str, arg: Option<&String>) -> anyhow::Result<u8> {
    let arg = arg.context(format!("Missing {} argument", name))?;
    FromStr::from_str(arg).context(format!("Failed to parse {} from {}", name, arg))
}
