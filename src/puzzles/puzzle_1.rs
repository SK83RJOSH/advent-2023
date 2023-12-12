use aho_corasick::AhoCorasick;
use anyhow::anyhow;
use itertools::Itertools;

pub use super::{PuzzleEntry, Result};

pub const PUZZLE: PuzzleEntry = (
    &[puzzle_part_1, puzzle_part_2],
    include_str!("puzzle_1.txt"),
);

const RADIX: usize = 10;
const NUMERIC_DIGITS: [&str; RADIX] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn puzzle_part_1(input: &str) -> Result<String> {
    let result: Result<usize> = input
        .lines()
        .map(|l| {
            let digits = l.chars().filter(char::is_ascii_digit).collect_vec();
            match (digits.first(), digits.last()) {
                (Some(a), Some(b)) => {
                    let as_digit = |c: &char| {
                        NUMERIC_DIGITS
                            .iter()
                            .position(|&m| m == format!("{c}"))
                            .ok_or(anyhow!("character was not a digit: {:?}", c))
                    };
                    Ok(as_digit(a)? * RADIX + as_digit(b)?)
                }
                (_, _) => Err(anyhow!("line does not contain any digits: {:?}", l)),
            }
        })
        .sum();
    Ok(result?.to_string())
}

const ALPHABETIC_DIGITS: [&str; RADIX] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn puzzle_part_2(input: &str) -> Result<String> {
    let group = NUMERIC_DIGITS
        .iter()
        .chain(ALPHABETIC_DIGITS.iter())
        .collect_vec();
    let ac = AhoCorasick::new(group)?;
    let result: Result<usize> = input
        .lines()
        .map(|l| {
            let digits = ac
                .find_overlapping_iter(l)
                .map(|m| m.pattern().as_usize() % RADIX)
                .collect_vec();
            match (digits.first(), digits.last()) {
                (Some(a), Some(b)) => Ok(a * RADIX + b),
                (_, _) => Err(anyhow!("line does not contain any digits: {:?}", l)),
            }
        })
        .sum();
    Ok(result?.to_string())
}
