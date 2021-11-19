#[derive(Debug)]
/// The token enum -- holds all tokens before they get turned into
/// an AST.
pub enum Token {
	/// Any values enclosed in [].
	/// E.g: [32, 32, "Array!"]
	Array(Vec<Token>),
	/// Any tokens enclosed in ().
	/// E.g: (32, 32, "String!", [32, 32, "Array!"])
	/// or   (10 + -3)
	Group(Vec<Token>),
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
	///Bracket closing `)`
	CloseBracket,
	/// Comma separator `,`
	Comma,
}