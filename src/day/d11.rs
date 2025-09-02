use std::collections::{HashMap, VecDeque};

use crate::{
    harness::{Day, Solution, SolutionInput},
    parse,
};

pub struct D11;

impl Day for D11 {
    type P1<'a> = P1;

    type P2<'a> = P2;

    fn day() -> u8 {
        11
    }
}

pub struct P1;

impl<'a> Solution<'a> for P1 {
    type Input = Vec<u64>;

    type Output = u64;

    fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut count = 0;

        let mut queue: VecDeque<(u64, u64)> = input.iter().map(|n| (*n, 0)).collect();
        while let Some((stone, blinks)) = queue.pop_front() {
            if blinks == 25 {
                count += 1;
                continue;
            }

            if stone == 0 {
                queue.push_back((1, blinks + 1));
            } else if let Some((left, right)) = split_digits(stone) {
                queue.push_back((left, blinks + 1));
                queue.push_back((right, blinks + 1));
            } else {
                queue.push_back((stone * 2024, blinks + 1));
            }
        }

        Ok(count)
    }
}

pub struct P2;

impl<'a> Solution<'a> for P2 {
    type Input = Vec<u64>;

    type Output = usize;

    fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
        let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
        Ok(input
            .iter()
            .map(|stone| dfs_count(*stone, 75, &mut cache))
            .sum())
    }
}

fn dfs_count(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(num_stones) = cache.get(&(stone, blinks)) {
        return *num_stones;
    }

    let num_stones = if stone == 0 {
        dfs_count(1, blinks - 1, cache)
    } else if let Some((left, right)) = split_digits(stone) {
        dfs_count(left, blinks - 1, cache) + dfs_count(right, blinks - 1, cache)
    } else {
        dfs_count(stone * 2024, blinks - 1, cache)
    };

    cache.insert((stone, blinks), num_stones);

    num_stones
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    if n == 0 {
        return None;
    }

    let digits = n.ilog10() + 1;
    if digits % 2 != 0 {
        return None;
    }

    let left = n / 10_u64.pow(digits / 2);
    let right = n % 10_u64.pow(digits / 2);
    Some((left, right))
}

impl<'a> SolutionInput<'a> for Vec<u64> {
    fn read(mut reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let mut v = vec![];
        for token in buf.trim().split_ascii_whitespace() {
            v.push(parse::parse_u64(token)?);
        }

        Ok(v)
    }
}
