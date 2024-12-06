use grid::{Grid, Point};

use crate::harness::{iter, Day, SolutionInput};

pub struct D6;

impl Day for D6 {
    type P1<'a> = p1::P1;
    type P2<'a> = p2::P2;

    fn day() -> u8 {
        6
    }
}

pub mod p1 {
    use std::collections::HashSet;

    use crate::harness::Solution;

    use super::{grid::Point, Map};

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Map;
        type Output = usize;

        fn solve(mut input: Self::Input) -> crate::harness::Result<Self::Output> {
            let mut visited: HashSet<Point> = HashSet::new();

            while let Some(prev_pos) = input.move_guard() {
                visited.insert(prev_pos);
            }

            Ok(visited.len())
        }
    }
}

pub mod p2 {
    use crate::harness::Solution;

    use super::Map;

    pub struct P2;

    impl<'a> Solution<'a> for P2 {
        type Input = Map;
        type Output = usize;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            todo!()
        }
    }
}

pub struct Map {
    guard: Option<(Point, Direction)>,
    grid: Grid<MapItem>,
}

impl Map {
    pub fn new(grid: Grid<MapItem>) -> Self {
        let mut guard: Option<(Point, Direction)> = None;

        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                let point = Point::new(x as isize, y as isize);
                if let Some(MapItem::Guard(dir)) = grid.get(&point) {
                    guard.replace((point, *dir));
                }
            }
        }

        Self { guard, grid }
    }

    /// Move the guard, returning their previous position
    pub fn move_guard(&mut self) -> Option<Point> {
        let (g_point, g_dir) = if let Some(g) = self.guard {
            g
        } else {
            return None;
        };

        let point_facing = g_point
            + match g_dir {
                Direction::North => Point::new(0, -1),
                Direction::East => Point::new(1, 0),
                Direction::South => Point::new(0, 1),
                Direction::West => Point::new(-1, 0),
            };

        match self.grid.get(&point_facing) {
            Some(MapItem::Empty) | Some(MapItem::Guard(_)) => {
                self.guard.replace((point_facing, g_dir));
            }
            Some(MapItem::Obstacle) => {
                self.guard.replace((g_point, g_dir.turn()));
                return self.move_guard();
            }
            None => {
                self.guard.take();
            }
        };

        Some(g_point)
    }
}

impl<'a> SolutionInput<'a> for Map {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let rows: Vec<Vec<MapItem>> = iter::lines(reader)
            .map(|r| r.map(|line| parse_row(&line)))
            .collect::<Result<_, _>>()?;
        Ok(Map::new(Grid::new(rows)))
    }
}

fn parse_row(line: &str) -> Vec<MapItem> {
    line.chars()
        .map(|c| match c {
            '#' => MapItem::Obstacle,
            '^' => MapItem::Guard(Direction::North),
            '>' => MapItem::Guard(Direction::East),
            'v' => MapItem::Guard(Direction::South),
            '<' => MapItem::Guard(Direction::West),
            _ => MapItem::Empty,
        })
        .collect()
}

pub enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn(&self) -> Self {
        match self {
            &Self::North => Self::East,
            &Self::East => Self::South,
            &Self::South => Self::West,
            &Self::West => Self::North,
        }
    }
}

mod grid {
    use std::ops::{Add, Mul};

    pub struct Grid<T> {
        rows: Vec<Vec<T>>,
    }

    impl<T> Grid<T> {
        pub fn new(rows: Vec<Vec<T>>) -> Self {
            let expected_cols = rows.first().map_or(0, |row| row.len());
            for (row_num, row) in rows.iter().enumerate() {
                let row_len = row.len();
                assert_eq!(
                    expected_cols, row_len,
                    "Expected row {} to have {} elements, but has {}",
                    row_num, expected_cols, row_len
                );
            }
            Self { rows }
        }

        pub fn rows(&self) -> usize {
            self.rows.len()
        }

        pub fn cols(&self) -> usize {
            self.rows.first().map_or(0, |row| row.len())
        }

        pub fn get(&self, idx: &Point) -> Option<&T> {
            if idx.x < 0 || idx.y < 0 {
                return None;
            }
            let x = idx.x as usize;
            let y = idx.y as usize;
            let row = self.rows.get(y)?;
            row.get(x)
        }
    }

    impl<T> From<Vec<Vec<T>>> for Grid<T> {
        fn from(value: Vec<Vec<T>>) -> Self {
            Self::new(value)
        }
    }

    impl<T> FromIterator<Vec<T>> for Grid<T> {
        fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
            Self::new(iter.into_iter().collect())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Point {
        x: isize,
        y: isize,
    }

    impl Point {
        pub fn new(x: isize, y: isize) -> Self {
            Self { x, y }
        }

        pub fn on_grid<T>(&self, grid: &Grid<T>) -> bool {
            self.x > 0
                && (self.x as usize) < grid.cols()
                && self.y > 0
                && (self.y as usize) < grid.rows()
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
}
