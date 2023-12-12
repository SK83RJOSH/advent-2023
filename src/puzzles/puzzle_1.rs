use anyhow::anyhow;
use itertools::Itertools;

pub use super::{PuzzleEntry, Result};

pub const PUZZLE: PuzzleEntry = (&[puzzle], include_str!("puzzle_1.txt"));

pub fn puzzle(input: &str) -> Result<String> {
    let lines: Result<Vec<u32>> = input
        .lines()
        .map(|l| {
            let digits = l.chars().filter(char::is_ascii_digit).collect_vec();
            match (digits.first(), digits.last()) {
                (Some(a), Some(b)) => {
                    const RADIX: u32 = 10;
                    let as_digit = |c: &char| {
                        c.to_digit(RADIX)
                            .ok_or(anyhow!("character was not a digit: {:?}", c))
                    };
                    Ok(as_digit(a)? * RADIX + as_digit(b)?)
                }
                (_, _) => Err(anyhow!("line does not contain two digits: {:?}", l)),
            }
        })
        .collect();
    let result: u32 = lines?.iter().sum();
    Ok(result.to_string() + "\n")
}
