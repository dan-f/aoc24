use crate::{
    harness::{Day, InputError},
    parse,
};

pub struct D2;

impl Day for D2 {
    type P1 = p1::P1;
    type P2 = p1::P1;

    fn num() -> u8 {
        2
    }
}

mod p1 {
    use crate::harness::{Solution, SolutionInput};

    use super::Report;

    pub struct P1;

    impl Solution for P1 {
        type Input = Vec<Report>;
        type Output = usize;

        fn solve(input: Self::Input) -> crate::harness::SolveResult<Self::Output> {
            Ok(input.iter().filter(|report| report.is_safe()).count())
        }
    }

    impl SolutionInput for Vec<Report> {
        fn read(reader: impl std::io::BufRead) -> Result<Self, crate::harness::InputError> {
            let mut reports: Vec<_> = vec![];

            for line in reader.lines() {
                let line = line?;
                let nums: Vec<_> = line.split_whitespace().collect();
                reports.push(Report::parse(nums)?);
            }

            Ok(reports)
        }
    }
}

pub struct Report(Vec<u8>);

impl Report {
    fn is_safe(&self) -> bool {
        let levels = &self.0;
        if levels.len() < 2 {
            return true;
        }

        let mut prv = levels[0];
        let inc = levels[1] >= prv;
        let allowed_diff = 1..=3;

        for cur in self.0.iter().skip(1).copied() {
            if cur > prv && !inc || cur < prv && inc || cur == prv {
                return false;
            }

            if !allowed_diff.contains(&cur.abs_diff(prv)) {
                return false;
            }

            prv = cur;
        }

        true
    }

    fn parse(input: Vec<&str>) -> Result<Self, InputError> {
        let mut levels: Vec<_> = vec![];

        for n in input {
            let n = parse::parse_u8(n)?;
            levels.push(n);
        }

        Ok(Report(levels))
    }
}

#[cfg(test)]
mod test {
    use super::Report;

    #[test]
    fn test_is_safe() {
        let inputs = [
            (Report(vec![7, 6, 4, 2, 1]), true),
            (Report(vec![1, 2, 7, 8, 9]), false),
            (Report(vec![9, 7, 6, 2, 1]), false),
            (Report(vec![1, 3, 2, 4, 5]), false),
            (Report(vec![8, 6, 4, 4, 1]), false),
            (Report(vec![1, 3, 6, 7, 9]), true),
        ];

        for (report, expected_safe) in inputs {
            assert_eq!(expected_safe, report.is_safe())
        }
    }
}
