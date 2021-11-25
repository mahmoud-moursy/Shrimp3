use std::fs::File;

use anyhow::Result;
use clap::Parser;

mod lexer;
mod errors;
mod tokens;
mod parser;
mod nodes;

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "T-O-R-U-S <bageliq@protonmail.com>")]
struct Cli {
	/// The Shrimp file to execute
	file: Option<String>,
	/// Boolean -- decides whether to display the tokens or not.
	/// Only for debugging purposes.
	#[clap(short, long)]
	display_tokens: bool,
}

fn main() -> Result<()> {

		let args = Cli::parse();
		
		// TODO: Use tokens in meaningful way.
	let mut tokens = lexer::make_tokens(
		File::open(
			args.file.unwrap_or(
				String::from("main.imp")
			).as_str()
		)
			.expect("Failed to open file. (Does it exist?)")
	)?;
	if args.display_tokens {
		println!("TOKENS:");
		println!("{:?}", tokens);
		println!("END TOKENS");
	}
	// TODO: Use nodes in meaningful way.
	let nodes = parser::parse(tokens)?;
	
	// This should NOT be here, and should NOT make it into
	// the final release.
	// TODO: Start properly debugging code?
	// Mmmm, nah...
	println!("{:?}", nodes);
	
	Ok(())
}
