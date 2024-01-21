use std::{collections::VecDeque, str::FromStr};

use aoc::input::day_04::INPUT;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    fst(INPUT);
    snd(INPUT);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: usize,
    winning_nums: Vec<usize>,
    my_nums: Vec<usize>,
}

impl Card {
    pub fn count_wins(&self) -> usize {
        itertools::iproduct!(&self.winning_nums, &self.my_nums)
            // note that using sets would be better here, but then we'd need to handle potential duplicates
            .filter(|(x, y)| x == y)
            .count()
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        static STRUCTURE_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Card +(?P<id>\d+):(?P<win>.+)\|(?P<mine>.+)").unwrap());
        static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<digits>\d+)").unwrap());
        let cap = STRUCTURE_RE.captures(line).expect(line);
        let id = cap["id"].parse().unwrap();
        let parse_ints = |s| {
            NUM_RE
                .captures_iter(s)
                .map(|cap| cap["digits"].parse().unwrap())
                .collect_vec()
        };
        let winning_nums = parse_ints(&cap["win"]);
        let my_nums = parse_ints(&cap["mine"]);
        Ok(Self {
            id,
            winning_nums,
            my_nums,
        })
    }
}

fn points_from_count(count: usize) -> usize {
    match count {
        0 => 0,
        n => 2_usize.pow((n - 1) as u32),
    }
}

fn fst(input: &str) {
    dbg!(input
        .lines()
        .map(|s| s.parse::<Card>().unwrap())
        .map(|card| points_from_count(card.count_wins()))
        .sum::<usize>());
}

fn snd(input: &str) {
    dbg!(
        input
            .lines()
            .map(|s| s.parse::<Card>().unwrap())
            // we fold down the collection scratch cards in order
            .fold(
                (0, VecDeque::from([1usize])),
                |(total, mut multiplier_stack), card| {
                    // the top of the stack (front) is always the multiplier for the current card
                    // if the stack is empty it's 1 because we have 1 copy of each card at
                    // the beginning
                    let current_card_multiplier = multiplier_stack.pop_front().unwrap_or(1);
                    let current_wins = card.count_wins();
                    // we realize the cards on the stack up to the point of the current card's influence
                    if multiplier_stack.len() < current_wins {
                        multiplier_stack.extend(vec![1; current_wins - multiplier_stack.len()]);
                    }
                    // we add the copies of the cards we just won by mutating the multipliers of the
                    // cards that are coming up
                    for multiplier in multiplier_stack.iter_mut().take(current_wins) {
                        *multiplier += current_card_multiplier;
                    }
                    // we have as many copies of the current card as its multiplier says
                    // so we add that to the total
                    (total + current_card_multiplier, multiplier_stack)
                }
            )
            .0
    );
}
