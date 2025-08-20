use std::collections::{HashMap, HashSet};

use crate::harness::{iter, Day, Solution, SolutionInput};

pub struct D8;

impl Day for D8 {
    type P1<'a> = P1;

    type P2<'a> = P2;

    fn day() -> u8 {
        8
    }
}

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Map;

    type Output = usize;

    fn solve(map: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut antinode_locs: HashSet<(isize, isize)> = HashSet::new();
        for (_, locs) in map.antennae_by_freq {
            for (i, (a1_col, a1_row)) in locs.iter().enumerate() {
                let (a1_col, a1_row) = (*a1_col as isize, *a1_row as isize);
                for j in i + 1..locs.len() {
                    let (a2_col, a2_row) = (locs[j].0 as isize, locs[j].1 as isize);
                    let d_col = a2_col - a1_col;
                    let d_row = a2_row - a1_row;
                    let antinode_1 = (a2_col + d_col, a2_row + d_row);
                    let antinode_2 = (a1_col - d_col, a1_row - d_row);
                    if (0..map.cols as isize).contains(&antinode_1.0)
                        && (0..map.rows as isize).contains(&antinode_1.1)
                    {
                        antinode_locs.insert(antinode_1);
                    }
                    if (0..map.cols as isize).contains(&antinode_2.0)
                        && (0..map.rows as isize).contains(&antinode_2.1)
                    {
                        antinode_locs.insert(antinode_2);
                    }
                }
            }
        }
        Ok(antinode_locs.len())
    }
}

pub struct P2;

impl<'a> Solution<'a> for P2 {
    type Input = Map;

    type Output = usize;

    fn solve(map: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut antinode_locs: HashSet<(isize, isize)> = HashSet::new();
        for (_, locs) in map.antennae_by_freq {
            for (i, (a1_col, a1_row)) in locs.iter().enumerate() {
                let (a1_col, a1_row) = (*a1_col as isize, *a1_row as isize);
                for j in i + 1..locs.len() {
                    let (a2_col, a2_row) = (locs[j].0 as isize, locs[j].1 as isize);
                    let d_col = a2_col - a1_col;
                    let d_row = a2_row - a1_row;

                    let mut antinode = (a1_col, a1_row);
                    while (0..map.cols as isize).contains(&antinode.0)
                        && (0..map.rows as isize).contains(&antinode.1)
                    {
                        antinode_locs.insert(antinode);
                        antinode.0 -= d_col;
                        antinode.1 -= d_row;
                    }

                    antinode = (a2_col, a2_row);
                    while (0..map.cols as isize).contains(&antinode.0)
                        && (0..map.rows as isize).contains(&antinode.1)
                    {
                        antinode_locs.insert(antinode);
                        antinode.0 += d_col;
                        antinode.1 += d_row;
                    }
                }
            }
        }
        Ok(antinode_locs.len())
    }
}

pub struct Map {
    cols: usize,
    rows: usize,
    antennae_by_freq: HashMap<char, Vec<(usize, usize)>>,
}

impl<'a> SolutionInput<'a> for Map {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let mut cols = 0;
        let mut rows = 0;
        let mut antennae_by_freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (row, line) in iter::lines(reader).enumerate() {
            let line = line?;
            rows += 1;
            cols = line.len();
            for (col, c) in line.char_indices() {
                if c != '.' {
                    antennae_by_freq.entry(c).or_default().push((col, row));
                }
            }
        }
        Ok(Map {
            cols,
            rows,
            antennae_by_freq,
        })
    }
}
