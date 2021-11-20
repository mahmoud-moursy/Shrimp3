use crate::tokens::Token;

/// This file is responsible for all nodes in the AST
pub enum Node {
	Term(Token),
	/// Any values enclosed in [].
	/// E.g: [32, 32, "Array!"]
	Array(Vec<Node>),
	/// Any tokens enclosed in ().
	/// E.g: (32, 32, "String!", [32, 32, "Array!"])
	/// or   (10 + -3)
	Group(Vec<Node>),
	/// Any call expression
	/// i.e: print("Hello world! 🦐")
	CallExpr {
		name: String,
		args: Vec<Token>,
	},
}