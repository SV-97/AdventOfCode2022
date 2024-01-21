#![feature(array_windows)]
//  applying the refactor suggested by this lint makes the code quite a bit less readable
#![allow(clippy::option_map_unit_fn)]

use std::str::FromStr;

use aoc::input::day_05::INPUT;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use pcw_fn::{Functor, PcwFn, VecPcwFn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<Range> for VecPcwFn<usize, Range> {
    fn from(value: Range) -> Self {
        let jumps = vec![value.source_start, value.source_end() + 1];
        let funcs = vec![
            Range {
                source_start: 0,
                dest_start: 0,
                len: value.source_start,
            },
            value,
            Range {
                source_start: value.source_end() + 1,
                dest_start: value.source_end() + 1,
                len: usize::MAX - (value.source_end() + 1),
            },
        ];
        VecPcwFn::try_from_iters(jumps, funcs).unwrap()
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
    fn dest_for(&self, source: usize) -> RangeOutput {
        let dist = source as i64 - self.source_start as i64;
        if dist >= 0 && dist <= self.len as i64 {
            RangeOutput::InRange(self.dest_start + usize::try_from(dist).unwrap())
        } else {
            RangeOutput::OutRange(source)
        }
    }

    /// Compose the functions induced by two ranges
    fn compose(&self, other: &Self) -> Vec<Self> {
        let f = VecPcwFn::from(*self);
        let g = VecPcwFn::from(*other);
        let gcl = g.clone().fmap(|range| move |x| range.dest_for(x));
        let combined_jumps = f
            .clone()
            .combine::<_, _, VecPcwFn<_, _>, _>(g.clone(), |_, _| ())
            .into_jumps();
        f.combine::<_, _, VecPcwFn<_, _>, _>(g, |l, r| Range {
            source_start: l.source_start,
            dest_start: gcl.eval(l.dest_start).inner(),
            len: l.len,
        })
        .into_funcs()
        .collect_vec()
    }
}

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
    dbg!(&seeds);
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
    dbg!(&seeds);
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
    /*dbg!(seeds
    .iter()
    .map(|&seed| { maps.iter().fold(seed, |current, map| map.dest_for(current)) })
    .min());*/
}
