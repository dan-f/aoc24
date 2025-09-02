use std::collections::HashSet;

use crate::harness::{iter, Day, Solution, SolutionInput};

pub struct D12;

impl Day for D12 {
    type P1<'a> = P1;

    type P2<'a> = P1;

    fn day() -> u8 {
        12
    }
}

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Map;

    type Output = usize;

    fn solve(map: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut total_fence_price = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if !visited.contains(&(row, col)) {
                    let region = find_region(&map, row, col);
                    total_fence_price += region.fence_price();
                    visited.extend(region.plots.iter());
                }
            }
        }

        Ok(total_fence_price)
    }
}

fn find_region(map: &Map, row: usize, col: usize) -> Region {
    let c = map.plots[row][col];
    let mut plots = HashSet::new();
    let mut perimeter = 0;

    let mut to_explore = vec![(row, col)];
    while let Some((row, col)) = to_explore.pop() {
        if plots.contains(&(row, col)) {
            continue;
        }

        plots.insert((row, col));

        if row > 0 && map.plots[row - 1][col] == c {
            to_explore.push((row - 1, col));
        } else {
            perimeter += 1;
        }
        if row < map.rows() - 1 && map.plots[row + 1][col] == c {
            to_explore.push((row + 1, col));
        } else {
            perimeter += 1;
        }
        if col > 0 && map.plots[row][col - 1] == c {
            to_explore.push((row, col - 1));
        } else {
            perimeter += 1;
        }
        if col < map.cols() - 1 && map.plots[row][col + 1] == c {
            to_explore.push((row, col + 1));
        } else {
            perimeter += 1;
        }
    }

    Region { plots, perimeter }
}

struct Region {
    plots: HashSet<(usize, usize)>,
    perimeter: usize,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn fence_price(&self) -> usize {
        self.area() * self.perimeter
    }
}

pub struct Map {
    plots: Vec<Vec<char>>,
}

impl Map {
    pub fn rows(&self) -> usize {
        self.plots.len()
    }

    pub fn cols(&self) -> usize {
        if let Some(row) = self.plots.get(0) {
            row.len()
        } else {
            0
        }
    }
}

impl<'a> SolutionInput<'a> for Map {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        Ok(Map {
            plots: iter::lines(reader)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|line| line.chars().collect())
                .collect(),
        })
    }
}
