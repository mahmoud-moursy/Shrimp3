use thiserror::Error;

use crate::tokens::Token;

/// Stores all user-made error types
#[derive(Error, Debug)]
pub enum Err {
	#[error("Unexpected EOL in line {0} at char {1}")]
	UnexpectedEOL(usize, usize),
	#[error("Unexpected EOF in line {0} at char {1}")]
	UnexpectedEOF(usize, usize),
	#[error("Unexpected character `{2}` in line {0} at char {1}")]
	UnexpectedChar(usize, usize, char),
	#[error("Found an unexpected token near {:?}", match.0 {
	Some(token) => token.to_string(),
	None => "the end of file.".to_string()
	})]
	UnexpectedToken(Option<Token>),
}