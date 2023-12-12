use std::ops::{Index, IndexMut};

use anyhow::{anyhow, Context};
use itertools::Itertools;

pub use super::{PuzzleEntry, Result};

pub const PUZZLE: PuzzleEntry = (
    &[puzzle_part_1, puzzle_part_2],
    include_str!("puzzle_2.txt"),
);

#[inline(always)]
const fn cube3(red: usize, green: usize, blue: usize) -> Cube3 {
    Cube3::new(red, green, blue)
}

#[derive(Clone, Copy)]
struct Cube3 {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Cube3 {
    #[inline(always)]
    pub const fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    #[inline(always)]
    pub fn index(color: &str) -> Result<usize> {
        const COLORS: [&str; 3] = ["red", "green", "blue"];
        COLORS
            .iter()
            .position(|&v| v == color)
            .context(format!("invalid color {color:?}"))
    }

    #[inline(always)]
    pub fn contains(&self, rhs: Self) -> bool {
        self.red >= rhs.red && self.green >= rhs.green && self.blue >= rhs.blue
    }

    #[inline(always)]
    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Index<usize> for Cube3 {
    type Output = usize;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Cube3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            _ => panic!("index out of bounds"),
        }
    }
}

fn split_pair(input: &str, pattern: char) -> Result<[&str; 2]> {
    if let Ok(parts) = input.split(pattern).collect_vec().try_into() {
        Ok(parts)
    } else {
        Err(anyhow!("failed to split {input:?} by {pattern:?}"))
    }
}

fn puzzle_part_1(input: &str) -> Result<usize> {
    const CUBE_COUNT: Cube3 = cube3(12, 13, 14);
    input
        .lines()
        .map(|line| -> Result<_> {
            let [game, turns] = split_pair(line.trim_start_matches("Game "), ':')?;
            let game: usize = game.parse()?;
            let results = turns.split(';').map(|turn| -> Result<bool> {
                let mut result = cube3(0, 0, 0);
                for value in turn.split(',') {
                    let [count, color] = split_pair(value.trim(), ' ')?;
                    let count: usize = count.parse()?;
                    let index = Cube3::index(color)?;
                    result[index] += count;
                }
                Ok(CUBE_COUNT.contains(result))
            });
            Ok((game, results))
        })
        .filter_map_ok(|(game, mut results)| results.all(|v| v.is_ok_and(|v| v)).then_some(game))
        .sum()
}

fn puzzle_part_2(input: &str) -> Result<usize> {
    input
        .lines()
        .map(|line| -> Result<_> {
            let [_, turns] = split_pair(line.trim_start_matches("Game "), ':')?;
            let mut result = cube3(0, 0, 0);
            for value in turns.split([';', ',']) {
                let [count, color] = split_pair(value.trim(), ' ')?;
                let count: usize = count.parse()?;
                let index = Cube3::index(color)?;
                result[index] = result[index].max(count);
            }
            Ok(result.power())
        })
        .sum()
}
