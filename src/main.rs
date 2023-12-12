use anyhow::Context;
use clap::Parser;

use puzzles::PUZZLES;

mod puzzles;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 2)]
    day: usize,

    #[arg(short, long, default_value_t = 2)]
    part: usize,

    #[arg(short, long)]
    input: Option<String>,
}

fn main() -> puzzles::Result<()> {
    let args = Args::parse();
    let (functions, default) = PUZZLES
        .get(args.day - 1)
        .context(format!("invalid day: {:?}", args.day))?;
    let function = functions
        .get(args.part - 1)
        .context(format!("invalid part: {:?}", args.part))?;
    let input = args.input.unwrap_or(default.to_string());
    let output = function(&input)?;
    println!("{output}");
    Ok(())
}
