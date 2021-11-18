use std::fs::File;

use anyhow::Result;
use clap::{AppSettings, Parser};

mod lexer;
mod errors;
mod tokens;

#[derive(Debug, Parser)]
struct Cli {
	/// The Shrimp file to execute
	file: String,

}

fn main() -> Result<()> {
	let args = Cli::parse();

	// TODO: Use tokens in meaningful way.
	let tokens = lexer::make_tokens(
		File::open(&args.file)
			.expect("Failed to open file. (Does it exist?)")
	)?;
	Ok(())
}
