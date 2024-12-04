use std::ops::{Add, Mul};

use once_cell::sync::Lazy;

use crate::harness::{input, iter, Day, SolutionInput};

pub struct D4;

impl Day for D4 {
    type P1<'a> = p1::P1;
    type P2<'a> = p2::P2;

    fn day() -> u8 {
        4
    }
}

pub mod p1 {
    use std::collections::HashSet;

    use crate::harness::Solution;

    use super::{Crossword, Point};

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Crossword;
        type Output = usize;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            let mut visited_xs: HashSet<Point> = HashSet::new();

            let mut count = 0;
            for point in input.points() {
                let xs: Vec<_> = input
                    .find_xs(point)
                    .into_iter()
                    .filter(|p| !visited_xs.contains(p))
                    .collect();
                for p in xs {
                    count += input.words_from_point("XMAS", p);
                    visited_xs.insert(p);
                }
            }

            Ok(count)
        }
    }
}

pub mod p2 {
    use crate::{day::d4::Point, harness::Solution};

    use super::Crossword;

    pub struct P2;

    impl<'a> Solution<'a> for P2 {
        type Input = Crossword;
        type Output = usize;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            Ok(input
                .points()
                .filter(|p| input.get(*p) == Some('A'))
                .filter(|p| is_x_mas(&input, p))
                .count())
        }
    }

    fn is_x_mas(cw: &Crossword, point: &Point) -> bool {
        if cw.get(*point) != Some('A') {
            return false;
        }

        let nw = *point + Point::new(-1, -1);
        let ne = *point + Point::new(1, -1);
        let se = *point + Point::new(1, 1);
        let sw = *point + Point::new(-1, 1);

        ((cw.get(nw) == Some('M') && cw.get(se) == Some('S'))
            || (cw.get(nw) == Some('S') && cw.get(se) == Some('M')))
            && ((cw.get(sw) == Some('M') && cw.get(ne) == Some('S'))
                || (cw.get(sw) == Some('S') && cw.get(ne) == Some('M')))
    }
}

pub struct Crossword(Vec<Vec<char>>);

impl Crossword {
    const DIRS: Lazy<[Point; 8]> = Lazy::new(|| {
        [
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(1, 1),
            Point::new(-1, 1),
            Point::new(-1, -1),
            Point::new(1, -1),
        ]
    });

    fn find_xs(&self, point: Point) -> Vec<Point> {
        let points = match self.get(point) {
            Some('X') => vec![point],
            Some('M') => Self::DIRS.iter().map(|p| point + *p).collect(),
            Some('A') => Self::DIRS.iter().map(|p| point + *p * 2).collect(),
            Some('S') => Self::DIRS.iter().map(|p| point + *p * 3).collect(),
            _ => vec![],
        };

        points
            .into_iter()
            .filter(|p| self.get(*p) == Some('X'))
            .collect()
    }

    fn words_from_point(&self, word: &str, point: Point) -> usize {
        Self::DIRS
            .into_iter()
            .filter(|delta| {
                let has_word = self.has_word(word, 0, point, *delta);
                has_word
            })
            .count()
    }

    fn has_word(&self, word: &str, word_idx: usize, point: Point, delta: Point) -> bool {
        let to_match = if let Some(c) = word.chars().nth(word_idx) {
            c
        } else {
            return true;
        };

        let cur = if let Some(c) = self.get(point) {
            c
        } else {
            return false;
        };

        if cur != to_match {
            return false;
        }

        self.has_word(word, word_idx + 1, point + delta, delta)
    }

    fn get(&self, idx: Point) -> Option<char> {
        if idx.x < 0 || idx.y < 0 {
            return None;
        }
        let x = idx.x as usize;
        let y = idx.y as usize;
        let row = self.0.get(y)?;
        row.get(x).copied()
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.0.iter().enumerate().flat_map(|(col_i, row)| {
            row.iter()
                .enumerate()
                .map(move |(row_i, _)| Point::new(row_i as isize, col_i as isize))
        })
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day::d4::Point;

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

        assert!(!crossword.has_word("xmas", 0, Point::new(0, 0), Point::new(1, 0)));
        assert!(!crossword.has_word("xmas", 0, Point::new(5, 0), Point::new(1, 0)));

        assert!(crossword.has_word("XMAS", 0, Point::new(2, 0), Point::new(1, 1)));
        assert!(crossword.has_word("XMAS", 0, Point::new(4, 1), Point::new(-1, 0)));
        assert!(crossword.has_word("XMAS", 0, Point::new(0, 3), Point::new(1, 0)));
        assert!(crossword.has_word("XMAS", 0, Point::new(1, 4), Point::new(0, -1)));
    }

    #[test]
    fn test_word_count() {
        let crossword = Crossword(vec![
            vec!['.', '.', '.', 'S', '.', '.', '.'],
            vec!['.', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', '.', 'M', '.', '.', '.'],
            vec!['.', '.', '.', 'X', '.', '.', '.'],
            vec!['.', '.', '.', 'M', '.', '.', '.'],
            vec!['.', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', '.', 'S', '.', '.', '.'],
        ]);

        assert_eq!(2, crossword.words_from_point("XMAS", Point::new(3, 3)));
    }
}
