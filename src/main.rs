use std::fs::File;

use anyhow::Result;
use clap::Parser;

mod data_types;
mod errors;
mod interpreter;
mod lexer;
mod nodes;
mod parser;
mod std_lib;
mod tokens;

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
    let tokens = lexer::make_tokens(
        File::open(args.file.unwrap_or(String::from("main.imp")).as_str())
            .expect("Failed to open file. (Does it exist?)"),
    )?;

    if args.display_tokens {
        println!("TOKENS:");
        println!("{:?}", tokens);
        println!("END TOKENS");
    }
    let nodes = parser::parse(tokens)?;

    let nodes = parser::make_fn(nodes)?;

    let nodes = parser::make_fn_call(nodes)?;

    interpreter::interpret(nodes)?;

    Ok(())
}
