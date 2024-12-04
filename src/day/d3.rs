use once_cell::sync::Lazy;
use regex::Regex;

use crate::{harness::Day, parse::parse_u32};

pub struct D3;

impl Day for D3 {
    type P1<'a> = p1::P1;
    type P2<'a> = p1::P1;

    fn day() -> u8 {
        3
    }
}

pub mod p1 {
    use crate::harness::{Solution, SolutionInput};

    use super::find_muls;

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Box<dyn Iterator<Item = (u32, u32)> + 'a>;
        type Output = u32;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            Ok(input.map(|(x, y)| x * y).sum())
        }
    }

    impl<'a> SolutionInput<'a> for Box<dyn Iterator<Item = (u32, u32)> + 'a> {
        fn read(reader: impl std::io::BufRead + 'a) -> Result<Self, crate::harness::InputError> {
            let it = reader
                .lines()
                .map(Result::unwrap)
                .flat_map(|line| find_muls(&line));

            Ok(Box::new(it))
        }
    }
}

fn find_muls(haystack: &str) -> Vec<(u32, u32)> {
    static MUL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    MUL_RE
        .captures_iter(haystack)
        .map(|c| c.extract())
        .map(|(_, [x, y])| (parse_u32(x).unwrap(), parse_u32(y).unwrap()))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day::d3::find_muls;

    #[test]
    fn test_parse_muls() {
        let haystack = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], find_muls(haystack));
    }
}
