use std::ops::Add;

use crate::harness::{input, iter, Day, SolutionInput};

pub struct D4;

impl Day for D4 {
    type P1<'a> = p1::P1;
    type P2<'a> = p1::P1;

    fn day() -> u8 {
        4
    }
}

pub mod p1 {
    use crate::harness::Solution;

    use super::Crossword;

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Crossword;
        type Output = u32;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            todo!()
        }
    }
}

pub struct Crossword(Vec<Vec<char>>);

impl Crossword {
    pub fn has_word(&self, mut word: Vec<char>, point: Point, delta: PointDelta) -> bool {
        let to_match = if let Some(c) = word.pop() {
            c
        } else {
            return true;
        };

        let cur = if let Some(c) = self.get(point) {
            c
        } else {
            return false;
        };

        let next_point = if let Some(p) = point + delta {
            p
        } else {
            return false;
        };

        cur == to_match && self.has_word(word, next_point, delta)
    }

    pub fn get(&self, idx: Point) -> Option<char> {
        let row = self.0.get(idx.y)?;
        row.get(idx.x).copied()
    }
}

impl<'a> SolutionInput<'a> for Crossword {
    fn read(reader: impl std::io::BufRead + 'a) -> input::Result<Self> {
        let table: Vec<Vec<char>> = iter::lines(reader)
            .map(|line| line.map(|line| line.chars().collect::<Vec<_>>()))
            .collect::<input::Result<_>>()?;
        Ok(Crossword(table))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointDelta {
    x: isize,
    y: isize,
}

impl PointDelta {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<PointDelta> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: PointDelta) -> Self::Output {
        Some(Point {
            x: self.x.checked_add_signed(rhs.x)?,
            y: self.y.checked_add_signed(rhs.y)?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::day::d4::{Point, PointDelta};

    use super::Crossword;

    #[test]
    fn test_has_word() {
        let crossword = Crossword(vec![
            vec!['.', '.', 'X', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', 'A', '.', '.', 'A', '.'],
            vec!['X', 'M', 'A', 'S', '.', 'S'],
            vec!['.', 'X', '.', '.', '.', '.'],
        ]);

        let xmas = || "XMAS".chars().rev().collect::<Vec<_>>();

        assert!(!crossword.has_word(xmas(), Point::new(0, 0), PointDelta::new(1, 0)));
        assert!(!crossword.has_word(xmas(), Point::new(5, 0), PointDelta::new(1, 0)));

        assert!(crossword.has_word(xmas(), Point::new(2, 0), PointDelta::new(1, 1)));
        assert!(crossword.has_word(xmas(), Point::new(4, 1), PointDelta::new(-1, 0)));
        assert!(crossword.has_word(xmas(), Point::new(0, 3), PointDelta::new(1, 0)));
        assert!(crossword.has_word(xmas(), Point::new(1, 4), PointDelta::new(0, -1)));
    }
}
