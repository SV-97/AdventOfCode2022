#![feature(array_windows)]
//  applying the refactor suggested by this lint makes the code quite a bit less readable
#![allow(clippy::option_map_unit_fn)]

use std::{cmp::Ordering, str::FromStr};

use aoc::input::day_05::INPUT;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace().map(|s| s.parse().unwrap());
        Ok(Range {
            dest_start: it.next().unwrap(),
            source_start: it.next().unwrap(),
            len: it.next().unwrap(),
        })
    }
}

impl Range {
    pub fn source_end(&self) -> usize {
        self.source_start + self.len
    }

    pub fn dest_end(&self) -> usize {
        self.dest_start + self.len
    }
}

#[derive(Clone, Copy)]
enum RangeOutput {
    InRange(usize),
    OutRange(usize),
}

impl RangeOutput {
    pub fn inner(self) -> usize {
        match self {
            RangeOutput::InRange(x) => x,
            RangeOutput::OutRange(x) => x,
        }
    }
}

impl Range {
    /// Get the destination value for a given source value
    pub fn dest_for(&self, source: usize) -> RangeOutput {
        let dist = source as i64 - self.source_start as i64;
        if dist >= 0 && dist <= self.len as i64 {
            RangeOutput::InRange(self.dest_start + usize::try_from(dist).unwrap())
        } else {
            RangeOutput::OutRange(source)
        }
    }

    /// Truncate self's input to the given range (keeeping the action on that range intact)
    fn restrict(self, other: &Self) -> Self {

        todo!();
    }

    /// composes self with source inside of self's range.
    fn compose_inner(&self, source: &Self) -> Vec<Self> {
        // truncate source so its dest matches own source interval
        let source = {
            let dest_end = source.dest_end();
            let new_source_end = dest_end.min(self.source_end());
            let dest_start = source.dest_start;
            let new_source_start = 
        }
        // let dist_start = (source.dest_start as i64) - (self.source_start as i64);
        // let dist_end = (source.dest_end() as i64) - (self.source_end() as i64);
        match (
            source.dest_start.cmp(&self.source_start),
            source.dest_end().cmp(&self.source_end()),
        ) {
            // intervals are effectively equal
            (Ordering::Equal, Ordering::Equal)
            | (Ordering::Equal, Ordering::Greater)
            | (Ordering::Less, Ordering::Equal)
            | (Ordering::Less, Ordering::Greater) => vec![
                // return composition of both *on the interval of self*
                Range {
                    source_start: source.source_start,
                    dest_start: self.dest_start,
                    len: self.len,
                },
            ],
            (Ordering::Greater, Ordering::Equal) | (Ordering::Greater, Ordering::Greater) => {
                // self acts on the left of the interval, the composition on the right
                let left_len = source.dest_start - self.source_start;
                vec![
                    Range {
                        source_start: self.source_start,
                        dest_start: self.dest_start,
                        len: left_len,
                    },
                    Range {
                        source_start: self.source_start + left_len + 1,
                        dest_start: self.dest_for(source.dest_start).inner(),
                        len: self.len - left_len,
                    },
                ]
            }
            (Ordering::Equal, Ordering::Less) | (Ordering::Less, Ordering::Less) => {
                // self acts on the right of the interval, the composition on the left
                let right_len = self.source_end() - source.dest_end();
                let left_len = self.len - right_len;
                vec![
                    Range {
                        source_start: self.source_start,
                        dest_start: self
                            .dest_for(source.dest_for(self.source_start).inner())
                            .inner(),
                        len: left_len,
                    },
                    Range {
                        source_start: self.source_start + left_len + 1,
                        dest_start: self.dest_start + left_len + 1,
                        len: right_len,
                    },
                ]
            }
            (Ordering::Greater, Ordering::Less) => {
                // self acts on the left and right; the composition acts in the middle
                let right_len = self.source_end() - source.dest_end();
                let left_len = source.dest_start - self.source_start;
                let center_len = self.len - left_len - right_len;
                vec![
                    Range {
                        source_start: self.source_start,
                        dest_start: self.dest_start,
                        len: left_len,
                    },
                    Range {
                        source_start: self.source_start + left_len + 1,
                        dest_start: self
                            .dest_for(source.dest_for(self.source_start + left_len + 1).inner())
                            .inner(),
                        len: center_len,
                    },
                    Range {
                        source_start: self.source_start + left_len + center_len + 1,
                        dest_start: self.dest_start + left_len + center_len + 1,
                        len: right_len,
                    },
                ]
            }
        }
        .into_iter()
        .filter(|range| range.len != 0)
        .collect_vec()
    }

    /// Compose the functions induced by two ranges in the sense of self(source(x))
    pub fn compose(&self, source: &Self) -> Vec<Self> {
        // we first split the source range into at most three segments:
        // * left of self source range
        // * inside of it
        // * and to the right of it
        let left = if source.dest_start < self.source_start {
            // note that this condition can only be true if source_start != 0
            // so this subtraction is safe.
            // It also means that there has to be a left segment
            let end_bound = self.source_start - 1;
            let end = end_bound.min(source.dest_end());
            let len = end - source.dest_start;
            // the values in this interval are outside the interval where self
            // acts (differently from the identity).
            // So here only `source` acts on inputs.
            Some(Range {
                source_start: source.source_start,
                dest_start: source.dest_start,
                len,
            })
        } else {
            // we're starting either inside of the self range or to the right of it
            // -> there's no left interval
            None
        };
        // repeat the same thing for the right side
        let right = if source.dest_end() > self.source_end() {
            let len = source.len.min(source.dest_end() - self.source_end());
            let start = source.dest_end() - len;
            Some(Range {
                source_start: source.source_end() - len,
                dest_start: start,
                len,
            })
        } else {
            None
        };

        let mut inner = self.compose_inner(source);
        inner.extend(left);
        inner.extend(right);
        inner
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<Range>);

impl Map {
    fn dest_for(&self, source: usize) -> usize {
        for map in &self.0 {
            if let RangeOutput::InRange(res) = map.dest_for(source) {
                return res;
            }
        }
        source
    }

    fn compose(&self, other: &Self) -> Self {
        Self(
            itertools::iproduct!(&self.0, &other.0)
                .flat_map(|(left, right)| left.compose(right))
                .unique()
                .collect_vec(),
        )
    }
}

fn main() {
    fst(INPUT);
    snd(INPUT);
}

fn fst(input: &str) {
    static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<digits>\d+)").unwrap());
    let parse_ints = |s| {
        NUM_RE
            .captures_iter(s)
            .map(|cap| cap["digits"].parse::<usize>().unwrap())
    };
    let seeds = parse_ints(input.lines().next().unwrap()).collect_vec();
    // dbg!(&seeds);
    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|block| {
            Map(block
                .lines()
                .skip(1)
                .map(|line| line.parse::<Range>().unwrap())
                .collect_vec())
        })
        .collect_vec();
    dbg!(seeds
        .iter()
        .map(|&seed| { maps.iter().fold(seed, |current, map| map.dest_for(current)) })
        .min());
}

fn snd(input: &str) {
    static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<digits>\d+)").unwrap());
    let parse_ints = |s| {
        NUM_RE
            .captures_iter(s)
            .map(|cap| cap["digits"].parse::<usize>().unwrap())
    };
    let t = parse_ints(input.lines().next().unwrap()).collect_vec();
    let seeds = t
        .array_windows::<2>()
        .step_by(2)
        .map(|&[start, len]| Range {
            dest_start: start,
            source_start: 0,
            len,
        });
    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|block| {
            Map(block
                .lines()
                .skip(1)
                .map(|line| line.parse::<Range>().unwrap())
                .collect_vec())
        })
        .collect_vec();
    let result = maps
        .into_iter()
        .fold(Map(seeds.collect_vec()), |current, map| {
            map.compose(&current)
        });
    dbg!(result);
}
