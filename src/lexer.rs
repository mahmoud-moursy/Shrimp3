use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

use anyhow::bail;
use anyhow::Result;

use super::errors::Err;
use super::tokens::Token;

/// List of chars.
pub struct CharList(
	// List of chars to parse
	Vec<char>,
	// Index of chars.
	isize,
);

/// In case something bad happens :(
const CHAR_LIST_FAIL: &str = "ShrimpLang3's interpreter has had an internal error!\
(Failed to convert isize to usize in CharList)";

impl CharList {
	pub fn new(char_list: Vec<char>) -> Self {
		// isize must ALWAYS be negative one -- index fails otherwise
		Self(char_list, -1)
	}
	pub fn next(&mut self) -> Option<&char> {
		self.1 += 1;
		self.0.get(
			// Safely convert isize to usize.
			usize::try_from(self.1)
				.expect(CHAR_LIST_FAIL)
		)
	}
	pub fn current(&self) -> Option<&char> {
		self.0.get(
			// Safely convert isize to usize.
			usize::try_from(self.1)
				.expect(CHAR_LIST_FAIL)
		)
	}
	/// Get previous character in CharList
	pub fn previous(&mut self) -> Option<&char> {
		self.1 -= 1;
		self.0.get(
			// Safely convert isize to usize.
			usize::try_from(self.1)
				.expect(CHAR_LIST_FAIL)
		)
	}
}

// Generate all tokens for the parser to make an AST
pub fn make_tokens(mut file: File) -> Result<Vec<Token>> {
	// Track line position for use in error messages
	let mut line_pos: usize = 0;
	let mut line_num: usize = 0;

	// Get file contents
	let mut characs = vec![];
	file.read_to_end(&mut characs).unwrap();
	// Convert bytes into chars
	let temp: Vec<char> = characs.iter().map(|x| *x as char).collect();

	let mut charlist = CharList::new(temp);

	let mut out = vec![];

	while let Some(code) = charlist.next() {
		match code {
			'\n' => {
				line_num += 1;
				line_pos = 0;
			}
			'"' | '\'' => {
				// To know when to stop.
				let apostrophe_type = &code.clone();

				// Final output
				let mut out = String::new();

				// If the string isn't over, add current char
				while Some(apostrophe_type) != charlist.next() {
					out.push(
						match charlist.current() {
							Some(thing) => *thing,
							None => bail!(Err::UnexpectedEOF(line_num, line_pos))
						}
					)
				}
			}
			any => bail!(Err::UnexpectedToken(line_num, line_pos, *any))
		}
	}

	Ok(out)
}