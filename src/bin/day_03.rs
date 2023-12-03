#![feature(array_windows)]
//  applying the refactor suggested by this lint makes the code quite a bit less readable
#![allow(clippy::option_map_unit_fn)]

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};

use aoc::input::day_03::INPUT;
use itertools::Itertools;

fn main() {
    fst(INPUT);
    snd(INPUT);
}

struct Schematic {
    lines: Vec<SchematicLine>,
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Symbol {
    val: char,
    line_idx: usize,
    idx: usize,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
struct Number {
    val: usize,
    line_idx: usize,
    span: RangeInclusive<usize>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
enum SchematicEntry {
    Symbol(Symbol),
    Number(Number),
}

struct SchematicLine(Vec<SchematicEntry>);

impl SchematicLine {
    fn parse_line(line: &str, line_idx: usize) -> Self {
        #[derive(Default)]
        struct NumParser {
            current_num: u32,
            current_start_idx: Option<usize>,
        }
        impl NumParser {
            fn consume(self, digit: char, idx: usize) -> Self {
                NumParser {
                    current_num: 10 * self.current_num + digit.to_digit(10).unwrap(),
                    current_start_idx: if self.current_start_idx.is_some() {
                        self.current_start_idx
                    } else {
                        Some(idx)
                    },
                }
            }

            fn try_finish(self, idx: usize, line_idx: usize) -> Option<SchematicEntry> {
                self.current_start_idx.map(|start| {
                    SchematicEntry::Number(Number {
                        val: self.current_num as usize, // idx should be nonzero - yolo
                        span: start..=idx - 1,
                        line_idx,
                    })
                })
            }
        }
        let mut acc = vec![];
        line.chars()
            .enumerate()
            .fold(NumParser::default(), |state, (idx, c)| match c {
                d if d.is_ascii_digit() => state.consume(d, idx),
                '.' => {
                    state.try_finish(idx, line_idx).map(|span| acc.push(span));
                    // acc.push(SchematicEntry::Dot);
                    NumParser::default()
                }
                c => {
                    state.try_finish(idx, line_idx).map(|span| acc.push(span));
                    acc.push(SchematicEntry::Symbol(Symbol {
                        val: c,
                        idx,
                        line_idx,
                    }));
                    NumParser::default()
                }
            })
            .try_finish(line.len(), line_idx)
            .map(|span| acc.push(span));
        Self(acc)
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s
                .lines()
                .enumerate()
                .map(|(line_idx, line)| SchematicLine::parse_line(line, line_idx))
                .collect::<Vec<_>>(),
        })
    }
}

/// Computes the vertical L1 distance between a point and a line segment
fn vertical_l1_distance(point: usize, line: &RangeInclusive<usize>) -> usize {
    if line.contains(&point) {
        0
    } else if point < *line.start() {
        line.start() - point
    } else
    /* point > *line.end() */
    {
        point - line.end()
    }
}

impl Schematic {
    pub fn adjacencies(&self) -> HashMap<&Symbol, HashSet<&Number>> {
        use SchematicEntry as S;

        self.lines
            .array_windows::<2>()
            // get part nums via diagonal and vertical symbols
            .flat_map(|[top_line, bottom_line]| {
                itertools::iproduct!(&top_line.0, &bottom_line.0).flat_map(
                    move |(top, bot)| match (top, bot) {
                        (
                            S::Symbol(sym @ Symbol { idx, .. }),
                            S::Number(num @ Number { span, .. }),
                        )
                        | (
                            S::Number(num @ Number { span, .. }),
                            S::Symbol(sym @ Symbol { idx, .. }),
                        ) if vertical_l1_distance(*idx, span) <= 1 => Some((sym, num)),
                        _ => None,
                    },
                )
            })
            .chain(
                // ... and via horizontal symbols
                self.lines.iter().flat_map(|line| {
                    line.0
                        .array_windows::<2>()
                        .flat_map(move |[l, r]| match (l, r) {
                            (
                                S::Symbol(sym @ Symbol { idx, .. }),
                                S::Number(num @ Number { span, .. }),
                            )
                            | (
                                S::Number(num @ Number { span, .. }),
                                S::Symbol(sym @ Symbol { idx, .. }),
                            ) if vertical_l1_distance(*idx, span) <= 1 => Some((sym, num)),
                            _ => None,
                        })
                }),
            )
            .sorted_by_key(|(sym, _num)| *sym)
            .group_by(|(sym, _num)| *sym)
            .into_iter()
            .map(|(key, group)| (key, group.map(|(_, num)| num).collect::<HashSet<_>>()))
            .collect()
    }

    pub fn part_nums(&self) -> impl Iterator<Item = usize> + '_ {
        self.adjacencies()
            .into_iter()
            .flat_map(|(_, adj_nums)| adj_nums)
            .map(|Number { val, .. }| *val)
    }

    pub fn gear_ratios(&self) -> impl Iterator<Item = usize> + '_ {
        self.adjacencies()
            .into_iter()
            .filter_map(|(sym, adj_nums)| {
                try_get_gear(sym, &adj_nums).map(|(_, [num1, num2])| num1.val * num2.val)
            })
    }
}

fn try_get_gear<'a, 'b>(
    sym: &'a Symbol,
    adj_nums: &HashSet<&'b Number>,
) -> Option<(&'a Symbol, [&'b Number; 2])> {
    if sym.val == '*' && adj_nums.len() == 2 {
        let mut it = adj_nums.iter();
        Some((sym, [it.next().unwrap(), it.next().unwrap()]))
    } else {
        None
    }
}

fn fst(input: &str) {
    dbg!(Schematic::from_str(input)
        .unwrap()
        .part_nums()
        .sum::<usize>());
}

fn snd(input: &str) {
    dbg!(Schematic::from_str(input)
        .unwrap()
        .gear_ratios()
        .sum::<usize>());
}
