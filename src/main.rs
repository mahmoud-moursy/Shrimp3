use std::fs::File;

use anyhow::Result;
use clap::Parser;

mod lexer;
mod errors;
mod tokens;
mod parser;
mod nodes;

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
	
	println!("{:?}", tokens);
	Ok(())
}
