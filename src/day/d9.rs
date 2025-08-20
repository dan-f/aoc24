use crate::harness::Day;

pub struct D9;

impl Day for D9 {
    type P1<'a> = p1::P1;

    type P2<'a> = p1::P1;

    fn day() -> u8 {
        9
    }
}

pub mod p1 {
    use std::iter;

    use crate::harness::{input, InputError, Solution, SolutionInput};

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Disk;

        type Output = usize;

        fn solve(mut disk: Self::Input) -> crate::harness::Result<Self::Output> {
            fragment_disk(&mut disk);
            Ok(compute_checksum(&disk))
        }
    }

    fn fragment_disk(disk: &mut [Option<usize>]) {
        let mut l = 0;
        let mut r = disk.len() - 1;

        loop {
            while l < r && disk[l].is_some() {
                l += 1;
            }
            while r > l && disk[r].is_none() {
                r -= 1;
            }
            if l < r {
                disk[l] = disk[r].take();
            } else {
                break;
            }
        }
    }

    fn compute_checksum(disk: &Disk) -> usize {
        disk.iter()
            .enumerate()
            .map(|(i, block)| match block {
                Some(n) => i * (*n as usize),
                None => 0,
            })
            .sum()
    }

    type Disk = Vec<Option<usize>>;

    impl<'a> SolutionInput<'a> for Disk {
        fn read(mut reader: impl std::io::BufRead + 'a) -> input::Result<Self> {
            let mut input = String::new();
            reader.read_to_string(&mut input)?;
            let mut disk = vec![];

            for (i, c) in input.trim().char_indices() {
                let num_blocks = c.to_digit(10).ok_or(InputError::InvalidInput {
                    msg: format!("Non-digit char in input: '{}'", c),
                    source: None,
                })? as usize;

                let block = if i % 2 == 0 { Some(i / 2) } else { None };

                for b in iter::repeat_n(block, num_blocks) {
                    disk.push(b);
                }
            }

            Ok(disk)
        }
    }
}
