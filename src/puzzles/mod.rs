pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;
pub type Puzzle = (fn(&String) -> Result<String>, &'static str);

pub const PUZZLES: [Puzzle; 0] = [];
