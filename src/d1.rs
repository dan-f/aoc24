use crate::Day;

pub struct D1;

impl Default for D1 {
    fn default() -> Self {
        D1
    }
}

pub struct Solution;

impl Day for D1 {
    type Solution = Solution;

    fn input_fname(&self) -> &'static str {
        "1"
    }

    fn solve(&mut self, reader: &mut impl std::io::BufRead) -> Self::Solution {
        todo!()
    }

    fn fmt(&self, soln: &Self::Solution) -> String {
        todo!()
    }
}
