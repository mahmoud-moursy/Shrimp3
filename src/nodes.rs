use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
/// This file is responsible for all nodes in the AST
pub enum Node {
	/// The most basic Node.
	/// Literally just a token.
	Term(Token),
	/// Any values enclosed in [].
	/// E.g: [32, 32, "Array!"]
	Array(Vec<Node>),
	/// Any tokens enclosed in ().
	/// E.g: fn(32, 32, "String!", [32, 32, "Array!"])
	/// or   (add(10, -3))
	Group(Vec<Node>),
	/// Any tokens enclosed in {}.
	/// E.g: { print("Hello world!"); }
	Block(Vec<Node>),
	/// Any call expression
	/// i.e: print("Hello world! 🦐")
	CallExpr {
		name: String,
		args: Vec<Token>,
	},
	/// Any function declaration. The typically, last part to be parsed.
	/// ```
	/// @main() {
	/// 	print("Hello world!");
	/// }
	/// ```
	FunctionDecl {
		name: String,
		args: Vec<Node>,
		nodes: Vec<Node>,
	},
}