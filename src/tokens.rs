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
	Str(&'static str),
	/// Any int or float. 123, 1.23, 33.32
	Num(f32),
	/// Any letters not in a string.
	Ident(&'static str),
	/// Any function identifier.
	/// E.g: @main
	/// or	 @other_fn
	FunctionIdent(),
	/// Line ends (;)
	EndLine,
}