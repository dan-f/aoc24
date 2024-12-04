use std::{env::current_dir, fs::File, io::BufReader};

use super::{
    input::InputError,
    part::Part,
    solution::{Result, Solution},
};

/// Daily two-part exercise
pub trait Day {
    type P1<'a>: Solution<'a>;
    type P2<'a>: Solution<'a>;

    /// The day (1-indexed)
    fn day() -> u8;

    /// Run the solution for the given daily `part`
    fn run(part: Part) -> Result<String> {
        let mut input_path = current_dir().map_err(|err| InputError::from(err))?;
        input_path.push("input");
        input_path.push(format!("d{}p{}", Self::day(), part.num()));
        if !input_path.exists() {
            input_path.pop();
            input_path.push(format!("d{}", Self::day()));
        }

        let input_file = File::open(input_path).map_err(|err| InputError::from(err))?;
        let reader = BufReader::new(input_file);

        match part {
            Part::One => Self::P1::run(reader),
            Part::Two => Self::P2::run(reader),
        }
    }
}
