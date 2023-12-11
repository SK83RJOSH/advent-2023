pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;
pub type Puzzle = (fn(&str) -> Result<String>, &'static str);

mod puzzle_1;

pub const PUZZLES: [Puzzle; 1] = [puzzle_1::PUZZLE];
