use std::collections::HashSet;

use crate::harness::{iter, Day, Solution, SolutionInput};

pub struct D12;

impl Day for D12 {
    type P1<'a> = P1;

    type P2<'a> = P2;

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

pub struct P2;

impl<'a> Solution<'a> for P2 {
    type Input = Map;

    type Output = usize;

    fn solve(map: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut total_fence_price = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if !visited.contains(&(row, col)) {
                    let region = find_region(&map, row, col);
                    total_fence_price += region.bulk_price();
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
    let mut n_edges = vec![];
    let mut s_edges = vec![];
    let mut e_edges = vec![];
    let mut w_edges = vec![];

    let mut to_explore = vec![(row, col)];
    while let Some((row, col)) = to_explore.pop() {
        if plots.contains(&(row, col)) {
            continue;
        }

        plots.insert((row, col));

        if row > 0 && map.plots[row - 1][col] == c {
            to_explore.push((row - 1, col));
        } else {
            n_edges.push((row, col));
        }
        if row < map.rows() - 1 && map.plots[row + 1][col] == c {
            to_explore.push((row + 1, col));
        } else {
            s_edges.push((row, col));
        }
        if col > 0 && map.plots[row][col - 1] == c {
            to_explore.push((row, col - 1));
        } else {
            w_edges.push((row, col));
        }
        if col < map.cols() - 1 && map.plots[row][col + 1] == c {
            to_explore.push((row, col + 1));
        } else {
            e_edges.push((row, col + 1));
        }
    }

    n_edges.sort();
    s_edges.sort();
    e_edges.sort_by_key(|(row, col)| (*col, *row));
    w_edges.sort_by_key(|(row, col)| (*col, *row));

    Region {
        plots,
        n_edges,
        s_edges,
        e_edges,
        w_edges,
    }
}

struct Region {
    plots: HashSet<(usize, usize)>,
    n_edges: Vec<(usize, usize)>,
    s_edges: Vec<(usize, usize)>,
    e_edges: Vec<(usize, usize)>,
    w_edges: Vec<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.n_edges.len() + self.s_edges.len() + self.e_edges.len() + self.w_edges.len()
    }

    fn sides(&self) -> usize {
        let mut sides = 0;

        for edges in [&self.n_edges, &self.s_edges] {
            let mut last_plot = None;
            for (row, col) in edges.iter().copied() {
                if let Some((last_row, last_col)) = last_plot {
                    if row != last_row || col != last_col + 1 {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
                last_plot.replace((row, col));
            }
        }

        for edges in [&self.e_edges, &self.w_edges] {
            let mut last_plot = None;
            for (row, col) in edges.iter().copied() {
                if let Some((last_row, last_col)) = last_plot {
                    if col != last_col || row != last_row + 1 {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
                last_plot.replace((row, col));
            }
        }

        sides
    }

    fn fence_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn bulk_price(&self) -> usize {
        self.area() * self.sides()
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
