// Simply for storing all errors and their handling

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Err {
	#[error("Unexpected EOL in line {0} at char {1}")]
	UnexpectedEOL(usize, usize),
	#[error("Unexpected EOF in line {0} at char {1}")]
	UnexpectedEOF(usize, usize),
	#[error("Unexpected token `{2}` in line {0} at char {1}")]
	UnexpectedToken(usize, usize, char),
}