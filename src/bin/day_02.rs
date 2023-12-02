use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

use aoc::input::day_02::INPUT;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    game_id: usize,
    draws: Vec<HashMap<Color, usize>>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static GROUP_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Game (?P<game_id>\d+): (?P<tail>.+)$").unwrap());
        static DRAW_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?P<count>\d+) (?P<color>red|green|blue)").unwrap());

        let captures = GROUP_RE.captures(s).unwrap();
        let game_id: usize = captures["game_id"].parse().unwrap();
        let draws = captures["tail"]
            .split(';')
            .map(|block| {
                DRAW_RE
                    .captures_iter(block)
                    .map(|cap| {
                        (
                            cap["color"].parse().unwrap(),
                            cap["count"].parse::<usize>().unwrap(),
                        )
                    })
                    .collect()
            })
            .collect();
        Ok(Game { game_id, draws })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bag(HashMap<Color, usize>);

impl Game {
    /// Check whether this game is possible relative to a given magic bag:
    /// a game is possible if the total number for each color can actually
    /// be taken from the magic bag at once.
    pub fn is_possible(&self, bag: &Bag) -> bool {
        self.draws.iter().all(|m| {
            [Color::Red, Color::Green, Color::Blue]
                .into_iter()
                .all(|color| match (m.get(&color), bag.0.get(&color)) {
                    (Some(l), Some(r)) => l <= r,
                    _ => true,
                })
        })
    }

    /// Check whether this game is possible relative to a given magic bag:
    /// a game is possible if the total number for each color can actually
    /// be taken from the magic bag at once.
    pub fn minimal_possible_bag(&self) -> Bag {
        let mut out = hash_map!(
            Color::Red => 0usize,
            Color::Green => 0,
            Color::Blue => 0,
        );
        self.draws.iter().for_each(|m| {
            for (&color, &count) in m {
                out.entry(color)
                    .and_modify(|current| *current = count.max(*current))
                    .or_insert(count);
            }
        });
        Bag(out)
    }
}

impl Bag {
    pub fn power(&self) -> usize {
        self.0.values().product()
    }
}

fn main() {
    fst();
    snd();
}

fn fst() {
    // yes this could've been a simple Vec3 together with the other color stuff
    let allowed = Bag(hash_map!(
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    ));
    dbg!(INPUT
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .filter(|game| game.is_possible(&allowed))
        .map(|game| game.game_id)
        .sum::<usize>());
}

fn snd() {
    dbg!(INPUT
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .map(|game| game.minimal_possible_bag().power())
        .sum::<usize>());
}

mod macros {
    #[macro_export]
    /// Provides a convenient way to construct `HashMap`s
    macro_rules! hash_map {
        ($($key:expr => $val:expr),* $(,)?) => {
            HashMap::from([$(($key, $val)),*])
        };
    }
}
