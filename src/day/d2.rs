use crate::{
    harness::{Day, InputError, SolutionInput},
    parse,
};

pub struct D2;

impl Day for D2 {
    type P1 = p1::P1;
    type P2 = p2::P2;

    fn num() -> u8 {
        2
    }
}

mod p1 {
    use crate::harness::Solution;

    use super::Report;

    pub struct P1;

    impl Solution for P1 {
        type Input = Vec<Report>;
        type Output = usize;

        fn solve(input: Self::Input) -> crate::harness::SolveResult<Self::Output> {
            Ok(input
                .iter()
                .filter(|report| report.safety().is_safe())
                .count())
        }
    }
}

mod p2 {
    use crate::harness::Solution;

    use super::Report;

    pub struct P2;

    impl Solution for P2 {
        type Input = Vec<Report>;
        type Output = usize;

        fn solve(input: Self::Input) -> crate::harness::SolveResult<Self::Output> {
            Ok(input
                .iter()
                .filter(|report| report.safe_with_tolerance())
                .count())
        }
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

pub struct Report(Vec<u8>);

#[derive(Debug, PartialEq)]
pub enum SafetyReport {
    Safe,
    Unsafe(usize),
}

impl SafetyReport {
    pub fn is_safe(&self) -> bool {
        self == &SafetyReport::Safe
    }
}

impl Report {
    fn safety(&self) -> SafetyReport {
        let levels = &self.0;
        if levels.len() < 2 {
            return SafetyReport::Safe;
        }

        let mut prv = levels[0];
        let inc = levels[1] >= prv;
        let allowed_diff = 1..=3;

        for (idx, cur) in self.0.iter().skip(1).copied().enumerate() {
            if cur > prv && !inc || cur < prv && inc || cur == prv {
                return SafetyReport::Unsafe(idx);
            }

            if !allowed_diff.contains(&cur.abs_diff(prv)) {
                return SafetyReport::Unsafe(idx);
            }

            prv = cur;
        }

        SafetyReport::Safe
    }

    fn safe_with_tolerance(&self) -> bool {
        let idx = if let SafetyReport::Unsafe(idx) = self.safety() {
            idx
        } else {
            return true;
        };

        let report_without = |idx: usize| {
            Report(
                self.0
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != idx)
                    .map(|(_, lvl)| *lvl)
                    .collect::<Vec<_>>(),
            )
        };

        let mut reports = vec![report_without(idx)];
        if idx < self.0.len() - 1 {
            reports.push(report_without(idx + 1));
        }
        if idx > 0 {
            reports.push(report_without(idx - 1));
        }

        reports.iter().any(|r| r.safety().is_safe())
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
    use super::{Report, SafetyReport};

    #[test]
    fn test_is_safe() {
        let inputs = [
            (Report(vec![7, 6, 4, 2, 1]), SafetyReport::Safe, true),
            (Report(vec![1, 2, 7, 8, 9]), SafetyReport::Unsafe(1), false),
            (Report(vec![9, 7, 6, 2, 1]), SafetyReport::Unsafe(2), false),
            (Report(vec![1, 3, 2, 4, 5]), SafetyReport::Unsafe(1), true),
            (Report(vec![8, 6, 4, 4, 1]), SafetyReport::Unsafe(2), true),
            (Report(vec![1, 3, 6, 7, 9]), SafetyReport::Safe, true),
        ];

        for (report, expected_safety, expected_safe_with_tolerance) in inputs {
            assert_eq!(expected_safety, report.safety());
            assert_eq!(expected_safe_with_tolerance, report.safe_with_tolerance());
        }
    }
}
