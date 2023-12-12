pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;
pub type Puzzle = fn(&str) -> Result<String>;
pub type PuzzleEntry = (&'static [Puzzle], &'static str);

mod puzzle_1;

pub const PUZZLES: &[PuzzleEntry] = &[puzzle_1::PUZZLE];
