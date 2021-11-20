#[derive(Debug)]
/// The token enum -- holds all token variants.
/// - Very important for the AST.
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
	/// Comma separator `,`
	Comma,
}