use clap::Parser;

/// Run advent of code 2024 solutions
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    /// Day of the advent
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: u8,

    /// Part of the daily exercise
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2), default_value_t = 1)]
    pub part: u8,
}
