use std::array;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    harness::{iter, Day, SolutionInput},
    parse::parse_u32,
};

pub struct D5;

impl Day for D5 {
    type P1<'a> = p1::P1;
    type P2<'a> = p1::P1;

    fn day() -> u8 {
        5
    }
}

mod p1 {
    use crate::{day::d5::PageSet, harness::Solution};

    use super::{Input, PageMap, Rule, Update};

    pub struct P1;

    impl<'a> Solution<'a> for P1 {
        type Input = Input;
        type Output = u32;

        fn solve(input: Self::Input) -> crate::harness::Result<Self::Output> {
            Ok(input
                .updates
                .iter()
                .filter(|u| in_order(u, &input.rules))
                .map(|u| middle(u))
                .sum())
        }
    }

    fn in_order(update: &Update, all_rules: &Vec<Rule>) -> bool {
        let pages: PageSet = update.iter().copied().collect();
        let mut rules: PageMap<PageSet> = PageMap::new(); // page -> preceding pages
        for (pre, suc) in all_rules {
            if pages.has(pre) && pages.has(suc) {
                if !rules.has(suc) {
                    rules.add(suc, PageSet::new());
                }
                rules.get_mut(suc).unwrap().add(*pre);
            }
        }

        let mut preceding = PageSet::new();
        for page in update {
            if let Some(must_precede) = rules.get(page) {
                if must_precede.intersect_count(&preceding) != must_precede.len() {
                    return false;
                }
            }
            preceding.add(*page);
        }

        true
    }

    fn middle(update: &Update) -> u32 {
        update[update.len() / 2]
    }
}

const PAGES: usize = 100;

type Rule = (u32, u32);
type Update = Vec<u32>;

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl<'a> SolutionInput<'a> for Input {
    fn read(reader: impl std::io::BufRead + 'a) -> crate::harness::input::Result<Self> {
        let mut rules = vec![];
        let mut updates = vec![];

        static RULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\|(\d+)").unwrap());
        static UPDATE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

        for line in iter::lines(reader) {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            if let Some(caps) = RULE_RE.captures(&line) {
                let (_, [x, y]) = caps.extract();
                let x = parse_u32(x)?;
                let y = parse_u32(y)?;
                rules.push((x, y));
            } else {
                let mut update = vec![];
                for (_, [x]) in UPDATE_RE.captures_iter(&line).map(|c| c.extract()) {
                    update.push(parse_u32(x)?);
                }
                updates.push(update);
            }
        }

        Ok(Input { rules, updates })
    }
}

pub struct PageMap<T>([Option<T>; PAGES]);

impl<T> PageMap<T> {
    pub fn new() -> Self {
        Self(array::from_fn(|_| None))
    }

    pub fn add(&mut self, page: &u32, val: T) {
        self.0[*page as usize].replace(val);
    }

    pub fn del(&mut self, page: &u32) {
        self.0[*page as usize].take();
    }

    pub fn get(&self, page: &u32) -> Option<&T> {
        self.0[*page as usize].as_ref()
    }

    pub fn get_mut(&mut self, page: &u32) -> Option<&mut T> {
        self.0[*page as usize].as_mut()
    }

    pub fn has(&self, page: &u32) -> bool {
        self.get(page).is_some()
    }

    pub fn len(&self) -> u32 {
        self.0.iter().filter(|t| t.is_some()).count() as u32
    }

    pub fn intersect_count(&self, other: &PageMap<T>) -> u32 {
        let mut count = 0;

        for p in 0..PAGES {
            if self.has(&(p as u32)) && other.has(&(p as u32)) {
                count += 1;
            }
        }

        count
    }
}

pub struct PageSet(PageMap<()>);

impl PageSet {
    pub fn new() -> Self {
        Self(PageMap::new())
    }

    pub fn add(&mut self, page: u32) {
        self.0.add(&page, ());
    }

    pub fn del(&mut self, page: &u32) {
        self.0.del(page);
    }

    pub fn has(&self, page: &u32) -> bool {
        self.0.has(page)
    }

    pub fn len(&self) -> u32 {
        self.0.len()
    }

    pub fn intersect_count(&self, other: &PageSet) -> u32 {
        self.0.intersect_count(&other.0)
    }
}

impl FromIterator<u32> for PageSet {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut p = PageSet::new();

        for page in iter {
            p.add(page);
        }

        p
    }
}
