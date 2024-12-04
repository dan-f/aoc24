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
    pub fn word_count(&self, word: &str, point: Point) -> usize {
        let deltas = [
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(1, 1),
            Point::new(-1, 1),
            Point::new(-1, -1),
            Point::new(1, -1),
        ];

        deltas
            .into_iter()
            .filter(|delta| {
                let has_word = self.has_word(word, 0, point, *delta);
                println!("[debug] has word for delta {:?}? {}", delta, has_word);
                has_word
            })
            .count()
    }

    pub fn has_word(&self, word: &str, word_idx: usize, point: Point, delta: Point) -> bool {
        let to_match = if let Some(c) = word.chars().nth(word_idx) {
            c
        } else {
            return true;
        };

        let cur = if let Some(c) = self.get(point) {
            c
        } else {
            println!("early return from `cur`");
            return false;
        };

        if cur != to_match {
            println!(
                "early return from `cur != match` - cur: {}, to_match: {}",
                cur, to_match
            );
            return false;
        }

        println!("[has_word] pass - to_match: {}, cur: {}", to_match, cur);

        self.has_word(word, word_idx + 1, point + delta, delta)
    }

    pub fn get(&self, idx: Point) -> Option<char> {
        if idx.x < 0 || idx.y < 0 {
            return None;
        }
        let x = idx.x as usize;
        let y = idx.y as usize;
        let row = self.0.get(y)?;
        row.get(x).copied()
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

        assert_eq!(2, crossword.word_count("XMAS", Point::new(3, 3)));
    }
}
