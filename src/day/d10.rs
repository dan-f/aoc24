use std::collections::HashSet;

use crate::harness::{iter, Day, InputError, Solution, SolutionInput};

pub struct D10;

impl Day for D10 {
    type P1<'a> = P1;

    type P2<'a> = P1;

    fn day() -> u8 {
        10
    }
}

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Map;

    type Output = usize;

    fn solve(map: Self::Input) -> crate::harness::Result<Self::Output> {
        Ok(map.trailheads.iter().map(|th| map.compute_score(th)).sum())
    }
}

pub struct Map {
    tiles: Vec<Vec<u32>>,
    trailheads: Vec<(usize, usize)>,
}

impl Map {
    pub fn compute_score(&self, trailhead: &(usize, usize)) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: Vec<(usize, usize)> = vec![*trailhead];

        while let Some((row, col)) = stack.pop() {
            if visited.contains(&(row, col)) {
                continue;
            }
            visited.insert((row, col));

            let mut neighbors = vec![];
            if (row as isize) - 1 >= 0 {
                neighbors.push((row - 1, col));
            }
            if row + 1 < self.rows() {
                neighbors.push((row + 1, col));
            }
            if (col as isize) - 1 >= 0 {
                neighbors.push((row, col - 1));
            }
            if col + 1 < self.cols() {
                neighbors.push((row, col + 1));
            }
            stack.extend(
                neighbors
                    .iter()
                    .filter(|(n_r, n_col)| self.tiles[*n_r][*n_col] == self.tiles[row][col] + 1),
            );
        }

        visited
            .iter()
            .filter(|(row, col)| self.tiles[*row][*col] == 9)
            .count()
    }

    pub fn rows(&self) -> usize {
        self.tiles.len()
    }

    pub fn cols(&self) -> usize {
        self.tiles.get(0).map(|row| row.len()).unwrap_or(0)
    }
}

impl<'a> SolutionInput<'a> for Map {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let mut tiles = vec![];
        let mut trailheads = vec![];

        for (row_i, line) in iter::lines(reader).enumerate() {
            let mut row = vec![];
            for (col_i, c) in line?.char_indices() {
                let tile = c.to_digit(10).ok_or(InputError::InvalidInput {
                    msg: format!("Unexpected non-digit in input: '{}'", c),
                    source: None,
                })?;
                row.push(tile);
                if tile == 0 {
                    trailheads.push((row_i, col_i));
                }
            }
            tiles.push(row);
        }

        Ok(Map { tiles, trailheads })
    }
}
