use std::fmt::Formatter;

use Token::*;

#[derive(Debug, Clone, PartialEq)]
/// The token enum -- holds all token variants.
/// - Used in generating the AST.
pub enum Token {
	/// Any string. 'Hello world!', "Hello world!"
	/// or					'\'Hello world!\''
	Str(String),
	/// Any int or float. 123, 1.23, 33.32
	Num(f32),
	/// Any letters not in a string.
	Ident(String),
	/// Function token `@`.
	/// A.K.A At Symbol
	FunctionDecl,
	/// Line ends `;`
	EndLine,
	/// Curly brace opening `{`
	OpenCurly,
	/// Curly brace closing `}`
	CloseCurly,
	/// Bracket opening `(`
	OpenBracket,
	/// Bracket closing `)`
	CloseBracket,
	/// Square bracket opening `[`
	OpenSquare,
	/// Square bracket closing `]`
	CloseSquare,
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "`{}`", match self {
			// Each token's representation.
			Str(string) => string.clone(),
			Num(num) => num.to_string(),
			Ident(ident) => ident.clone(),
			// P	A  I	N 		A	U		 C H O C O L A T
			FunctionDecl => "@".to_string(),
			EndLine => ";".to_string(),
			OpenCurly => "{".to_string(),
			CloseCurly => "}".to_string(),
			OpenBracket => "(".to_string(),
			CloseBracket => ")".to_string(),
			OpenSquare => "[".to_string(),
			CloseSquare => "]".to_string(),
		})
	}
}