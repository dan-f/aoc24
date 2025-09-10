use std::fmt::{self, Write};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    harness::{iter, Day, InputError, Solution, SolutionInput},
    parse,
};

pub struct D14;

impl Day for D14 {
    type P1<'a> = P1;

    type P2<'a> = P1;

    fn day() -> u8 {
        14
    }
}

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Bathroom;

    type Output = u64;

    fn solve(mut input: Self::Input) -> crate::harness::Result<Self::Output> {
        input.tick_seconds(100);
        Ok(input.safety_factor())
    }
}

pub struct Bathroom {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Bathroom {
    pub fn tick_seconds(&mut self, seconds: usize) {
        for robot in &mut self.robots {
            let velocity = (
                robot.velocity.0 * (seconds as isize),
                robot.velocity.1 * (seconds as isize),
            );
            robot.pos.0 =
                (robot.pos.0 as isize + velocity.0).rem_euclid(self.width as isize) as usize;
            robot.pos.1 =
                (robot.pos.1 as isize + velocity.1).rem_euclid(self.height as isize) as usize;
        }
    }

    pub fn safety_factor(&self) -> u64 {
        let (mid_x, mid_y) = (self.width / 2, self.height / 2);
        let even_width = self.width % 2 == 0;
        let even_height = self.height % 2 == 0;
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bot_left = 0;
        let mut bot_right = 0;

        for robot in &self.robots {
            let (x, y) = robot.pos;

            let is_left = x < mid_x;
            let is_right = (even_width && x >= mid_x) || (!even_width && x > mid_x);
            let is_top = y < mid_y;
            let is_bot = (even_height && y >= mid_y) || (!even_height && y > mid_y);

            if is_top && is_left {
                top_left += 1;
            } else if is_top && is_right {
                top_right += 1;
            } else if is_bot && is_left {
                bot_left += 1;
            } else if is_bot && is_right {
                bot_right += 1;
            }
        }

        top_left * top_right * bot_left * bot_right
    }
}

impl fmt::Debug for Bathroom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = vec![vec![0; self.width]; self.height];

        for r in &self.robots {
            let (x, y) = r.pos;
            map[y][x] += 1;
        }

        for row in map {
            for tile in row {
                if tile > 0 {
                    write!(f, "{}", tile)?;
                } else {
                    write!(f, ".")?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Robot {
    pos: (usize, usize),
    velocity: (isize, isize),
}

impl<'a> SolutionInput<'a> for Bathroom {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        Ok(Bathroom {
            width: 101,
            height: 103,
            robots: iter::lines(reader)
                .map(|line_res| line_res.and_then(Robot::try_from))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<String> for Robot {
    type Error = InputError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"p=(?<p_x>\d+),(?<p_y>\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)").unwrap()
        });

        let caps = RE.captures(&s).ok_or(InputError::InvalidInput {
            msg: format!("Bad robot definition: {}", s),
            source: None,
        })?;

        Ok(Robot {
            pos: (
                parse::parse_usize(&caps["p_x"])?,
                parse::parse_usize(&caps["p_y"])?,
            ),
            velocity: (
                parse::parse_isize(&caps["v_x"])?,
                parse::parse_isize(&caps["v_y"])?,
            ),
        })
    }
}
