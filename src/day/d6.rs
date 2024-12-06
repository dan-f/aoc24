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
    use std::{collections::HashSet, iter};

    use crate::harness::Solution;

    use super::Map;

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Map;
        type Output = usize;

        fn solve(mut input: Self::Input) -> crate::harness::Result<Self::Output> {
            Ok(iter::from_fn(|| input.move_guard())
                .map(|(point, _)| point)
                .collect::<HashSet<_>>()
                .len())
        }
    }
}

pub mod p2 {
    use std::{collections::HashSet, iter};

    use crate::harness::Solution;

    use super::{Map, Position};

    pub struct P2;

    impl<'a> Solution<'a> for P2 {
        type Input = Map;
        type Output = usize;

        fn solve(mut input: Self::Input) -> crate::harness::Result<Self::Output> {
            let possible_placements: HashSet<_> = iter::from_fn(|| input.move_guard())
                .map(|(point, _)| point)
                .skip(1) // do not place at the guard's starting position
                .collect();

            Ok(possible_placements
                .iter()
                .filter(|point| {
                    input.reset();
                    input.place_obstacle(&point);
                    let creates_cycle = has_cycle(&mut input);
                    creates_cycle
                })
                .count())
        }
    }

    fn has_cycle(map: &mut Map) -> bool {
        let mut visited: HashSet<Position> = HashSet::new();

        while let Some(pos) = map.move_guard() {
            if visited.contains(&pos) {
                return true;
            }
            visited.insert(pos);
        }

        false
    }
}

type Position = (Point, Direction);

fn point_facing(pos: &Position) -> Point {
    pos.0
        + match pos.1 {
            Direction::North => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
        }
}

#[derive(Clone)]
pub struct Map {
    guard_origin: Position,
    cur_guard: Option<Position>,
    obstacle: Option<Point>,
    grid: Grid<MapItem>,
}

impl Map {
    pub fn new(grid: Grid<MapItem>) -> Self {
        let mut guard: Option<Position> = None;

        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                let point = Point::new(x as isize, y as isize);
                if let Some(MapItem::Guard(dir)) = grid.get(&point) {
                    guard.replace((point, *dir));
                }
            }
        }

        Self {
            guard_origin: guard.unwrap(),
            cur_guard: guard,
            obstacle: None,
            grid,
        }
    }

    /// Move the guard, returning their previous position. A move results in
    /// either:
    /// - the guard continuing in their cardinal direction
    /// - the guard turning 90 degrees
    /// - the guard moving off the grid
    pub fn move_guard(&mut self) -> Option<Position> {
        let guard_pos = if let Some(g) = self.cur_guard {
            g
        } else {
            return None;
        };
        let (guard_point, guard_dir) = guard_pos;

        let facing = point_facing(&guard_pos);
        match self.grid.get(&facing) {
            Some(MapItem::Empty) | Some(MapItem::Guard(_)) => {
                self.cur_guard.replace((facing, guard_dir));
            }
            Some(MapItem::Obstacle) => {
                self.cur_guard.replace((guard_point, guard_dir.turn()));
            }
            None => {
                self.cur_guard.take();
            }
        };

        Some((guard_point, guard_dir))
    }

    pub fn can_place_obstacle(&self, point: &Point) -> bool {
        point != &self.guard_origin.0
            && self.obstacle.is_none()
            && self
                .grid
                .get(point)
                .is_some_and(|item| item == &MapItem::Empty)
    }

    pub fn place_obstacle(&mut self, point: &Point) {
        if !self.can_place_obstacle(point) {
            return;
        }
        self.grid.set(point, MapItem::Obstacle);
        self.obstacle.replace(*point);
    }

    pub fn reset(&mut self) {
        self.cur_guard = Some(self.guard_origin);
        if let Some(obstacle_pos) = self.obstacle {
            self.grid.set(&obstacle_pos, MapItem::Empty);
            self.obstacle.take();
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn facing(&self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
        }
    }
}

mod grid {
    use std::ops::{Add, Mul};

    #[derive(Clone)]
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

        pub fn set(&mut self, idx: &Point, val: T) {
            if self.get(idx).is_none() {
                return;
            }
            self.rows[idx.y as usize][idx.x as usize] = val;
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
