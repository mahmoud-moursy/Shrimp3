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

// Generate all tokens for the parser to make an AST
pub fn make_tokens(mut file: File) -> Result<Vec<Token>> {
	// Track line position for use in error messages
	let mut line_pos: usize = 0;
	let mut line_num: usize = 0;
	
	// Get file contents
	let mut characs = String::new();
	file.read_to_string(&mut characs).unwrap();
	
	// CharList -- provides easy way to browse through chars.
	let mut charlist = characs.chars().into_iter().peekable();
	
	drop(file);
	
	let mut final_out = vec![];
	
	while let Some(code) = charlist.next() {
		match code {
			ident if ident.is_alphabetic() => {
				// Final output
				let mut out = code.to_string();
				
				// Loop will continue pushing all chars to out until
				// a non ascii-alphanumeric character is found.
				while let Some(code) = charlist.peek() {
					// Break loop if code is not alphanumeric
					if !code.is_ascii_alphanumeric() {
						break;
					}
					out.push(charlist.next().unwrap());
				}
				final_out.push(
					Token::Ident(out)
				);
			}
			num if num.is_numeric() => {
				let mut out = num.to_string();
				
				// Set to true if a `.` is detected.
				let mut float = false;
				
				while let Some(code) = charlist.peek() {
					if !code.is_numeric() {
						// If number is a float,
						// add '.' to be parsed.
						if float == false && *code == '.' {
							float = true;
							out.push(charlist.next().unwrap());
							continue;
						}
						break;
					}
					
					out.push(charlist.next().unwrap())
				}
				
				final_out.push(
					Token::Num(
						out.parse().unwrap()
					)
				)
			}
			'@' => final_out.push(Token::FunctionDecl),
			'{' => final_out.push(Token::OpenCurly),
			'}' => final_out.push(Token::CloseCurly),
			'(' => final_out.push(Token::OpenBracket),
			')' => final_out.push(Token::CloseBracket),
			',' => final_out.push(Token::Comma),
			// Handles strings
			'"' => {
				
				// Final output
				let mut out = String::new();
				
				// If the file isn't over, add current char
				while let Some(thing) = charlist.next() {
					// Stop if the apostrophe opened with
					if thing == '"' {
						break;
					}
					out.push(
						match thing {
							'\n' => {
								line_num += 1;
								line_pos = 0;
								'\n'
							}
							// Handle escape sequences
							'\\' =>
								match charlist.next() {
									Some(thing) => {
										match thing {
											'\n' => continue,
											'n' => '\n',
											'u' => '\x00',
											other => other
										}
									}
									None => bail!(Err::UnexpectedEOF(line_num, line_pos))
								}
							any => any
						}
					);
					line_pos += 1;
				}
				
				final_out.push(
					Token::Str(out)
				)
			}
			';' => final_out.push(Token::EndLine),
			'\n' => {
				line_num += 1;
				line_pos = 0;
			}
			any if any.is_whitespace() => { line_pos += 1; }
			any => bail!(Err::UnexpectedToken(line_num, line_pos, any))
		}
		line_pos += 1;
	}
	
	Ok(final_out)
}