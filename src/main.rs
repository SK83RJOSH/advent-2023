use anyhow::anyhow;
use clap::Parser;

use puzzles::PUZZLES;

mod puzzles;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: usize,

    #[arg(short, long, default_value_t = 1)]
    part: usize,

    #[arg(short, long)]
    input: Option<String>,
}

fn main() -> puzzles::Result<()> {
    let args = Args::parse();

    match PUZZLES.get(args.day - 1) {
        Some((functions, default)) => match functions.get(args.part - 1) {
            Some(function) => {
                let input = args.input.unwrap_or(default.to_string());
                let output = function(&input)?;
                println!("{output}");
                Ok(())
            }
            None => Err(anyhow!("invalid part: {:?}", args.part)),
        },
        None => Err(anyhow!("invalid day: {:?}", args.day)),
    }
}
