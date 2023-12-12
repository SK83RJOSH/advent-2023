pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;
pub type Puzzle = fn(&str) -> Result<usize>;
pub type PuzzleEntry = (&'static [Puzzle], &'static str);

mod puzzle_1;
mod puzzle_2;

pub const PUZZLES: &[PuzzleEntry] = &[puzzle_1::PUZZLE, puzzle_2::PUZZLE];
