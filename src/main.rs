use anyhow::anyhow;
use clap::Parser;

use puzzles::PUZZLES;

mod puzzles;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: usize,

    #[arg(short, long)]
    input: Option<String>,
}

fn main() -> puzzles::Result<()> {
    let args = Args::parse();

    match PUZZLES.get(args.day) {
        Some((function, default)) => {
            let input = args.input.unwrap_or(default.to_string());
            let output = function(&input)?;
            println!("{output}");
            Ok(())
        }
        None => Err(anyhow!("invalid day: {:?}", args.day)),
    }
}
