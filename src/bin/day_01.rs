use aoc::input::day_01::INPUT;

fn main() {
    dbg!(fst(INPUT));
    dbg!(snd(INPUT));
}

pub fn fst(raw: &'static str) -> usize {
    raw.lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_ascii_digit());
            let first = chars.next().unwrap();
            let last = chars.last().unwrap_or(first);
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

const NUMERALS: [&str; 10] = [
    "zero", // zero shouldn't actually be valid and yes we should handle this better but I'm lazy
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn is_numeral_prefix(s: &str) -> bool {
    NUMERALS.into_iter().any(|num| num.starts_with(s))
}

pub fn parse_digits(line: &str) -> impl Iterator<Item = usize> + '_ {
    /// Drops at least one characters and as many as necessary for current to be a valid prefix to a numeral
    fn drop_until_valid_numeral_prefix(current: &mut String) {
        for new_start in 1..current.len() {
            let tail = &current[new_start..];
            if is_numeral_prefix(tail) {
                // the current subview of the buffer is a prefix for some numeral:
                // we can proceed with this bit
                current.drain(..new_start);
                break;
            }
        }
    }

    line.chars()
        .scan(String::new(), |current, c| {
            current.push(c);
            match c {
                _ if c.is_ascii_digit() => {
                    current.clear();
                    // going through format here is super hacky and we could go through ascii but eh
                    Some(Some(format!("{c}").parse().unwrap()))
                }
                _ if is_numeral_prefix(current) => {
                    let mut val = None;
                    for (i, num) in NUMERALS.into_iter().enumerate() {
                        if num == current.as_str() {
                            val = Some(i);
                            break;
                        }
                    }
                    // we might or might not have found a fully parsed value at this point.
                    // If we found one we should strip characters from the buffer until we're
                    // left with a valid buffer again.
                    // This might mean potentially emptying it completely.
                    // We do it this way rather than simply calling clear to account for
                    // potentially overlapping words.
                    if val.is_some() {
                        drop_until_valid_numeral_prefix(current);
                    }

                    Some(val)
                }
                _ => {
                    // similarly to above: we want to drop characters off the front until we're
                    // left with another valid prefix
                    drop_until_valid_numeral_prefix(current);
                    Some(None)
                }
            }
        })
        .flatten()
}

pub fn snd(raw: &'static str) -> usize {
    raw.lines()
        .map(|line| {
            // we create an iterator over all the (parsed) digits in the line
            let mut digits = parse_digits(line);
            // pick out the first one
            let first = digits.next().unwrap();
            // and the last one in the remainder - if there's no more digits in
            // the remainder the first one is also the last one
            let last = digits.last().unwrap_or(first);
            // going through format here is hacky and we could go through simple
            // ascii instead but eh - I'm lazy
            let val: usize = format!("{first}{last}").parse().unwrap();
            // println!("{line} = {val}");
            val
        })
        .sum::<usize>()
}
